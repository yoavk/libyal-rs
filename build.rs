use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// For now we expect to find a `libfsntfs` dylib
fn main() {
//    let script_dir = env::current_dir().expect("Failed to get current_dir");
//    let libfsntfs = script_dir.join("libfsntfs");
//
//    Command::new("sh")
//        .arg("autogen.sh")
//        .current_dir(libfsntfs)
//        .stderr(Stdio::inherit())
//        .stdout(Stdio::inherit())
//        .spawn()
//        .expect("autogen failed");
//
//    let dst = autotools::Config::new("libfsntfs").reconf("--install").build();
    println!("cargo:rustc-link-lib=dylib=libfsntfs");
    println!("cargo:rustc-link-search=native=./libfsntfs");
}
