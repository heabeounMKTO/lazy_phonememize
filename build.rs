use std::env;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", dir);
    println!("cargo:rustc-link-search=native=./libs"); 
    println!("cargo:rustc-link-lib=static=phonememize");
    println!("cargo:rustc-link-lib=espeak-ng");
    println!("cargo:rerun-if-changed=./libs/libphonememize.a");
}
