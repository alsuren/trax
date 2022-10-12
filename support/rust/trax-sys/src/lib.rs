#[cxx::bridge(namespace = "trax")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper.h");

        type Metadata;
        fn new_metadata() -> UniquePtr<Metadata>;
        type Logging;
        fn new_logging() -> UniquePtr<Logging>;
        type Bounds;
        fn new_bounds() -> UniquePtr<Bounds>;
        type Handle;
        // fn new_handle(...) -> UniquePtr<Handle>;
        type Client;
        // fn new_client(...) -> UniquePtr<Client>;
        type Server;
        // fn new_server(...) -> UniquePtr<Server>;
        type Image;
        fn new_image() -> UniquePtr<Image>;
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
