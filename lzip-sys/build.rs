use std::env;

extern crate cc;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    cc::Build::new()
        .file("lzlib-1.13/lzlib.c")
        .out_dir(&out_dir)
        .compile("liblz.a");

    println!("cargo:root={}", out_dir);

    // Tell cargo to tell rustc to link the system lzlib shared library.
    println!("cargo:rustc-link-lib=lz");
}
