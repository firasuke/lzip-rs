use std::env;

extern crate cc;

fn main() {
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    // "lzlib.c" is only for shared builds
    cc::Build::new()
        .file("lzlib-1.13/lzlib.h")
        .file("lzlib-1.13/lzip.h")
        .file("lzlib-1.13/cbuffer.c")
        .file("lzlib-1.13/decoder.h")
        .file("lzlib-1.13/decoder.c")
        .file("lzlib-1.13/encoder_base.h")
        .file("lzlib-1.13/encoder_base.c")
        .file("lzlib-1.13/encoder.h")
        .file("lzlib-1.13/encoder.c")
        .file("lzlib-1.13/fast_encoder.h")
        .file("lzlib-1.13/fast_encoder.c")
        .compile("liblz.a");

    // Tell cargo to tell rustc to link the system lzlib shared library.
    println!("cargo:rustc-link-lib=lz");
}
