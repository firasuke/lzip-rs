// This won't build for now as no ".pc" file is provided with lzlib
fn main() {
    pkg_config::Config::new().probe("lzip").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
