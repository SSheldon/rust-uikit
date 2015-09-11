extern crate objc_uikit;

use std::ptr;

use objc_uikit::UIApplicationMain;

#[no_mangle]
pub extern fn run_app() {
    unsafe {
        UIApplicationMain(0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    }
}
