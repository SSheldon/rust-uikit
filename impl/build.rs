extern crate gcc;

fn main() {
    gcc::compile_library("librust_uikit_impl.a",
        &["extern/RustApplicationDelegate.m"]);
}
