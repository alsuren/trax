use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    safety!(unsafe_ffi)
    // generate!(Metadata);
    // generate!(Logging);
    generate!(Bounds);
    // generate!(Handle);
    // generate!(Client);
    // generate!(Server);
    // generate!(Image);
    // generate!(Region);
    // generate!(Properties);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_a_constant() {
        assert_eq!(TRAX_ERROR, -1)
    }
}
