fn main() {
    // Tell cargo to tell rustc to link the system lzip
    // shared library.
    println!("cargo:rustc-link-lib=lz");
}
