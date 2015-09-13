use objc::Message;
use objc::runtime::Class;
use objc_foundation::INSObject;

use IUIView;

pub struct UIWindow {
    _private: (),
}

unsafe impl Message for UIWindow { }

impl INSObject for UIWindow {
    fn class() -> &'static Class {
        Class::get("UIWindow").unwrap()
    }
}

impl IUIView for UIWindow { }

impl UIWindow {
    pub fn make_key_and_visible(&self) {
        unsafe {
            msg_send![self, makeKeyAndVisible]
        }
    }
}
