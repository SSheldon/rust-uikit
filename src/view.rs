use objc::Message;
use objc::runtime::Class;
use objc_id::ShareId;
use objc_foundation::INSObject;

use UIColor;

pub trait IUIView : INSObject {
    fn background_color(&self) -> ShareId<UIColor> {
        unsafe {
            let obj: *mut UIColor = msg_send![self, backgroundColor];
            ShareId::from_ptr(obj)
        }
    }

    fn set_background_color(&self, color: ShareId<UIColor>) {
        unsafe {
            msg_send![self, setBackgroundColor:&*color]
        }
    }
}

pub struct UIView {
    _private: (),
}

unsafe impl Message for UIView { }

impl INSObject for UIView {
    fn class() -> &'static Class {
        Class::get("UIView").unwrap()
    }
}

impl IUIView for UIView { }
