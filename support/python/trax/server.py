"""
Bindings for the TraX sever.
"""

import collections

from ctypes import byref, cast, py_object

from . import TraxException, TraxStatus, Properties, HandleWrapper, ConsoleLogger, FileLogger
from .internal import \
        trax_metadata_create, trax_server_setup, \
        trax_logger_setup, trax_image_list_p, \
        trax_image_list_get, trax_metadata_release, \
        trax_server_wait, trax_server_reply, \
        trax_image_list_release, trax_region_p, \
        trax_properties_p, trax_logger, trax_terminate
from .image import ImageChannel, Image
from .region import Region

def wrap_image_list(list):

    channels = [ImageChannel.COLOR, ImageChannel.DEPTH, ImageChannel.IR]
    wrapped = {}

    for channel in channels:
        img = trax_image_list_get(list, ImageChannel.encode(channel))
        if not img:
            continue
        wrapped[channel] = Image.wrap(img)

    return wrapped

class Request(collections.namedtuple('Request', ['type', 'image', 'region', 'properties'])):

    """ A container class for client requests. Contains fileds type, image, region and parameters. """

def _logger(buf, len, obj):

    self = cast(obj, py_object)

    #self._log(string)

class Server(object):

    """ TraX server."""

    def __init__(self, region_formats, image_formats, image_channels=["color"], trackerName="", trackerDescription="", trackerFamily="", customMetadata=None, log=False):

        if isinstance(log, bool) and log:
            self._logger = trax_logger(ConsoleLogger())
        elif isinstance(log, str):
            self._logger = trax_logger(FileLogger(log))
        else:
            self._logger = None

        mdata = trax_metadata_create(Region.encode_list(region_formats),
            Image.encode_list(image_formats), ImageChannel.encode_list(image_channels),
            trackerName.encode('utf-8'), trackerDescription.encode('utf-8'), trackerFamily.encode('utf-8'))

        if isinstance(customMetadata, dict):
            custom = Properties(mdata.contents.custom, False)
            for key, value in customMetadata.items():
                custom.set(key, value)

        logger = trax_logger_setup(self._logger, None, 0)

        self._handle = HandleWrapper(trax_server_setup(mdata, logger))

        trax_metadata_release(byref(mdata))

    def wait(self):
        """ Wait for client message request. Recognize it and parse them when received .

            :returns: A request structure
            :rtype: trax.server.Request
        """

        timage = trax_image_list_p()
        tregion = trax_region_p()
        tproperties = trax_properties_p()

        status = TraxStatus.decode(trax_server_wait(self._handle.reference, byref(timage), byref(tregion), tproperties))

        if status == TraxStatus.QUIT:
            properties = Properties(tproperties)
            return Request(status, None, None, properties)

        if status == TraxStatus.INITIALIZE:
            image = wrap_image_list(timage)
            trax_image_list_release(byref(timage))
            region = Region.wrap(tregion)
            properties = Properties(tproperties)
            return Request(status, image, region, properties)

        if status == TraxStatus.FRAME:
            image = wrap_image_list(timage)
            trax_image_list_release(byref(timage))
            properties = Properties(tproperties)
            return Request(status, image, None, properties)

        else:
            return Request(TraxStatus.ERROR, None, None, None)

    def status(self, region, properties=None):
        """ Reply to client with a status region and optional properties.


            :param trax.region.Region region: Resulting region object.
            :param dict properties: Optional arguments as a dictionary.
        """
        assert(isinstance(region, Region))
        tproperties = Properties(properties)
        return TraxStatus.decode(trax_server_reply(self._handle.reference, cast(region.reference, trax_region_p), tproperties.reference))

    def __enter__(self):
        """ To support instantiation with 'with' statement. """
        return self

    def __exit__(self, *args, **kwargs):
        """ Destructor used by 'with' statement. """
        self.quit()

    def quit(self, reason=None):
        """ Sends quit message and end terminates communication. """
        if not reason is None:
            trax_terminate(self._handle.reference, reason.encode('utf-8'))
        self._handle = None
