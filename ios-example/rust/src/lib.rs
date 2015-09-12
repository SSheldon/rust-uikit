extern crate objc_foundation;
extern crate objc_uikit;
extern crate objc_uikit_impl;

use std::ptr;

use objc_foundation::{INSObject, INSString, NSString};
use objc_uikit::UIApplicationMain;
use objc_uikit_impl::ApplicationDelegate;

#[no_mangle]
pub extern fn run_app() {
    let class = ApplicationDelegate::class();
    let class_name = NSString::from_str(class.name());
    let class_name_ptr = &*class_name as *const _ as *mut _;

    unsafe {
        UIApplicationMain(0, ptr::null_mut(), ptr::null_mut(), class_name_ptr);
    }
}
