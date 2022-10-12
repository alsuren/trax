use std::env;
use std::path::PathBuf;

fn main() {
    // See https://rust-lang.github.io/rust-bindgen/tutorial-0.html and
    // https://docs.rs/cmake/latest/cmake/ for details of how this all works.

    // FIXME: the cmake config for the C++ bindings is probably simple enough that we can skip it and use the rust `cc` crate to drive compilation.
    // FIXME: this relative path feels like it is going be problematic when we try to publish it
    let dst = cmake::Config::new("../../../").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=trax");

    cxx_build::bridge("src/lib.rs") // returns a cc::Build
        // .file("src/demo.cc")
        .flag_if_supported("-std=c++14")
        .include(dst.join("include"))
        .include("src")
        .compile("cxxbridge-demo");
}
