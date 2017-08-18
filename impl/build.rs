extern crate gcc;
use std::env;

#[cfg(any(target_arch="i386", target_arch="x86", target_arch="i686", target_arch="x86_64"))]
fn sdk_path() -> String {
    "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk/".to_string()
}

#[cfg(any(target_arch="arm", target_arch="armv7", target_arch="armv7s", target_arch="aarch64"))]
fn sdk_path() -> String {
    "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/".to_string()
}

#[cfg(not(any(target_os = "ios", target_os="macos")))]
fn sdk_path() -> String {
    eprintln!("Invalid OS specified for target: {}", env::var("TARGET").unwrap());
    panic!()
}

fn main() {
    let include_location = sdk_path();

    gcc::Build::new()
    .include(&include_location)
    .file("extern/RustApplicationDelegate.m")
    .compile("librust_uikit_impl.a");
}
