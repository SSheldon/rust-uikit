use objc::Message;
use objc::runtime::Class;
use objc_foundation::INSObject;

pub trait IUIView : INSObject {
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
