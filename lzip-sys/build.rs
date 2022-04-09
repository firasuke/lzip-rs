extern crate cc;

fn main() {
    cc::Build::new()
        .file("lzlib-1.13/lzlib.c")
        .compile("liblz.a");

    // Tell cargo to tell rustc to link the system lzlib shared library.
    println!("cargo:rustc-link-lib=lz");
}
