#[cxx::bridge(namespace = "trax")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper.h");

        type Metadata;
        fn new_metadata() -> UniquePtr<Metadata>;
        fn image_formats(self: &Metadata) -> i32;
        fn region_formats(self: &Metadata) -> i32;
        // FIXME: the CXX crate doesn't like this return type. Wants it wrapped in UniquePtr or something.
        // fn tracker_name(self: &Metadata) -> CxxString;
        // fn tracker_description(self: &Metadata) -> CxxString;
        // fn tracker_family(self: &Metadata) -> CxxString;

        type Logging;
        // FIXME: Logging(trax_logging logging) and Logging(trax_logger callback = NULL, void *data = NULL, int flags = 0)
        fn new_logging() -> UniquePtr<Logging>;

        type Bounds;
        // FIXME: Bounds(trax_bounds bounds) and Bounds(float left, float top, float right, float bottom)
        fn new_bounds() -> UniquePtr<Bounds>;

        // Handle's constructor is protected.
        type Handle;
        // FIXME: fn region_formats(self: &Handle) -> bool;
        // FIXME: fn get_error(self: &Handle) -> CxxString;
        // FIXME: fn is_alive(self: &Handle) -> bool;

        type Client;
        // FIXME: fn new_client(...) -> UniquePtr<Client>;
        // FIXME: fn wait(self: &Client, region: &Region, properties: &Properties) -> i32; // problem with constness of arguments?
        //  int initialize(const Image &image, const Region &region, const Properties &properties)
        // FIXME: problem with constness of arguments:
        // fn initialize(
        //     self: &Client,
        //     image: &ImageList,
        //     region: &Region,
        //     properties: &Properties,
        // ) -> i32;

        type Server;
        // FIXME: fn new_server(...) -> UniquePtr<Server>;
        type Image;
        fn new_image() -> UniquePtr<Image>;
        // FIXME: this is undocumented
        type ImageList;
        type Region;
        fn new_region() -> UniquePtr<Region>;
        type Properties;
        fn new_properties() -> UniquePtr<Properties>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_a_constant() {
        assert!(!ffi::new_bounds().is_null())
    }
}
