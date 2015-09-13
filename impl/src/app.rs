use objc::Message;
use objc::runtime::Class;
use objc_foundation::INSObject;
use objc_uikit::IUIApplicationDelegate;

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

impl IUIApplicationDelegate for ApplicationDelegate { }
