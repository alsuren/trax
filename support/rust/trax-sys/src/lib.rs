#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("../../../../include/trax.h");

        type Metadata;
        type Logging;
        type Bounds;
        type Handle;
        type Client;
        type Server;
        type Image;
        type Region;
        type Properties;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_a_constant() {
        assert_eq!(TRAX_ERROR, -1)
    }
}
