extern crate libc;
extern crate objc_foundation;

use std::env;
use std::ffi::CString;
use std::ptr;

use libc::{c_char, c_int};
use objc_foundation::{INSObject, INSString, NSString};

#[link(name = "UIKit", kind = "framework")]
extern {
    fn UIApplicationMain(argc: c_int, argv: *mut *const c_char,
            principalClassName: *mut NSString,
            delegateClassName: *mut NSString) -> c_int;
}

pub trait IUIApplicationDelegate : INSObject { }

pub fn application_main<T: IUIApplicationDelegate>() -> ! {
    let args: Vec<_> = env::args().map(|s| CString::new(s).unwrap()).collect();
    let mut arg_ptrs: Vec<_> = args.iter().map(|s| s.as_ptr()).collect();

    let class = T::class();
    let class_name = NSString::from_str(class.name());
    let class_name_ptr = &*class_name as *const _ as *mut _;

    unsafe {
        UIApplicationMain(arg_ptrs.len() as c_int, arg_ptrs.as_mut_ptr(),
            ptr::null_mut(), class_name_ptr);
    }
    unreachable!();
}
