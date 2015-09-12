extern crate objc;
extern crate objc_foundation;

use objc::Message;
use objc::runtime::Class;
use objc_foundation::INSObject;

extern {
    fn RustApplicationDelegateClass() -> *mut Class;
}

pub struct ApplicationDelegate {
    _private: (),
}

unsafe impl Message for ApplicationDelegate { }

impl INSObject for ApplicationDelegate {
    fn class() -> &'static Class {
        unsafe {
            let cls = RustApplicationDelegateClass();
            assert!(!cls.is_null());
            &*cls
        }
    }
}
