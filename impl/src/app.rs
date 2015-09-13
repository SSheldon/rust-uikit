use objc::Message;
use objc::runtime::Class;
use objc_foundation::INSObject;
use objc_uikit::{IUIApplicationDelegate, self};

pub trait ApplicationDelegate {
    fn did_finish_launching(&self) -> bool;
}

extern {
    fn RustApplicationDelegateClass() -> *mut Class;
}

struct ApplicationDelegateObj {
    _private: (),
}

unsafe impl Message for ApplicationDelegateObj { }

impl INSObject for ApplicationDelegateObj {
    fn class() -> &'static Class {
        unsafe {
            let cls = RustApplicationDelegateClass();
            assert!(!cls.is_null());
            &*cls
        }
    }
}

impl IUIApplicationDelegate for ApplicationDelegateObj { }

pub fn application_main<T: ApplicationDelegate>(delegate: T) -> ! {
    objc_uikit::application_main::<ApplicationDelegateObj>();
}
