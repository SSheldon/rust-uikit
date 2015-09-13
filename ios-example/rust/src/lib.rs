extern crate uikit;
extern crate uikit_impl;

use uikit_impl::{application_main, ApplicationDelegate};

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
