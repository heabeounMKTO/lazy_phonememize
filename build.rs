use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    // Run make first
    let output = Command::new("sh")
        .arg("-c")
        .arg("make")
        .output()
        .expect("Failed to execute command");
    
    if !output.status.success() {
        eprintln!("Make command failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    
    println!("cargo:rustc-link-search={}", manifest_dir.display());
    let libs_dir = manifest_dir.join("libs");
    println!("cargo:rustc-link-search={}", libs_dir.display());
    println!("cargo:rustc-link-search={}", libs_dir.join("lib").display());
    println!("cargo:rustc-link-search={}", libs_dir.join("bin").display());
    
    // Add PKG_CONFIG_PATH for the custom lib location, THAT WE BUILT FROM SOURCE
    let lib_pkg_config = libs_dir.join("lib").join("pkgconfig");
    if lib_pkg_config.exists() {
        println!("cargo:rustc-env=PKG_CONFIG_PATH={}", lib_pkg_config.display());
    }
    println!("cargo:rustc-link-lib=static=phonememize");
    println!("cargo:rustc-link-lib=espeak-ng");
    
    // Tell cargo to look for the header files in the custom include directory
    println!("cargo:include={}", libs_dir.join("include").display());
    
    // Tell cargo to rerun if any of these changes
    println!("cargo:rerun-if-changed=Makefile");
    println!("cargo:rerun-if-changed=libs/lib/libphonememize.a");
    println!("cargo:rerun-if-changed=libs/lib/libespeak-ng.a");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", libs_dir.join("lib").display());
}
