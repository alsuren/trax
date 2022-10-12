#include "trax.h"

// workarounds for https://github.com/dtolnay/cxx/issues/280
namespace trax {
    std::unique_ptr<trax::Metadata> new_metadata() {
        return std::make_unique<trax::Metadata>();
    };
    std::unique_ptr<trax::Logging> new_logging() {
        return std::make_unique<trax::Logging>();
    };
    std::unique_ptr<trax::Bounds> new_bounds() {
        return std::make_unique<trax::Bounds>();
    };

    // Handle constructor is protected
    // std::unique_ptr<trax::Handle> new_handle() {
    //     return std::make_unique<trax::Handle>();
    // };

    // Client constructor takes at least 2 arguments:
    //     Client(int input, int output, Logging logger);
    //     Client(int server, Logging logger,  int timeout = -1);
    // std::unique_ptr<trax::Client> new_client() {
    //     return std::make_unique<trax::Client>();
    // };

    // Server constructor requires 2 arguments: Server(Metadata metadata, Logging log);
    // std::unique_ptr<trax::Server> new_server() {
    //     return std::make_unique<trax::Server>();
    // };
    std::unique_ptr<trax::Image> new_image() {
        return std::make_unique<trax::Image>();
    };
    std::unique_ptr<trax::Region> new_region() {
        return std::make_unique<trax::Region>();
    };
    std::unique_ptr<trax::Properties> new_properties() {
        return std::make_unique<trax::Properties>();
    };
}
