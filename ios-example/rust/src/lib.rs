extern crate objc_uikit;
extern crate objc_uikit_impl;

use objc_uikit_impl::{application_main, ApplicationDelegate};

struct ExampleAppDelegate;

impl ApplicationDelegate for ExampleAppDelegate {
    fn did_finish_launching(&self) -> bool {
        true
    }
}

#[no_mangle]
pub extern fn run_app() {
    application_main(ExampleAppDelegate);
}
