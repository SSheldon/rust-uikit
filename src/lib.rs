extern crate libc;
extern crate block;
#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

use libc::{c_char, c_int};
use objc_foundation::NSString;

#[link(name = "UIKit", kind = "framework")]
extern {
    pub fn UIApplicationMain(argc: c_int, argv: *mut *mut c_char,
            principalClassName: *mut NSString,
            delegateClassName: *mut NSString) -> c_int;
}
