use std::env;
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    // See https://rust-lang.github.io/rust-bindgen/tutorial-0.html and
    // https://docs.rs/cmake/latest/cmake/ for details of how this all works.

    // FIXME: this feels like it is going be problematic when we try to publish it
    let dst = cmake::Config::new("../../../")
        .define("MACOSX_RPATH", "FALSE")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=trax");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // autocxx stuff
    println!("cargo:WARNING=starting autocxx stuff now");
    eprintln!("cargo:WARNING=starting autocxx stuff now");
    let path = dst.join("include"); // include path
    let mut b = dbg!(autocxx_build::Builder::new("src/lib.rs", &[&path]).build())?;
    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++14")
        .compile("trax-autocxx-demo"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/lib.rs");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
