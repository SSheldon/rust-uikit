use objc::Message;
use objc::runtime::Class;
use objc_id::ShareId;
use objc_foundation::INSObject;

use UIView;

pub trait IUIViewController : INSObject {
    fn view(&self) -> ShareId<UIView> {
        unsafe {
            let view: *mut UIView = msg_send![self, view];
            ShareId::from_ptr(view)
        }
    }
}

pub struct UIViewController {
    _private: (),
}

unsafe impl Message for UIViewController { }

impl INSObject for UIViewController {
    fn class() -> &'static Class {
        Class::get("UIViewController").unwrap()
    }
}

impl IUIViewController for UIViewController { }
