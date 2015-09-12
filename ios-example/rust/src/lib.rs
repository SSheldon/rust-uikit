extern crate objc_uikit;
extern crate objc_uikit_impl;

use objc_uikit::application_main;
use objc_uikit_impl::ApplicationDelegate;

#[no_mangle]
pub extern fn run_app() {
    application_main::<ApplicationDelegate>();
}
