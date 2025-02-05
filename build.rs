use std::env;
use std::process::Command;
fn main() {
      let output = Command::new("sh")
        .arg("-c") // Use `-c` to pass a command string
        .arg("make")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", dir);
    println!("cargo:rustc-link-search=native=./libs"); 
    println!("cargo:rustc-link-lib=static=phonememize");
    println!("cargo:rustc-link-lib=espeak-ng");
    println!("cargo:rerun-if-changed=./libs/libphonememize.a");
}
