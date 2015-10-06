use objc::Message;
use objc::runtime::Class;
use objc_id::{Id, ShareId};
use objc_foundation::INSObject;

use {NoSyncSend, UIView};

pub trait IUIViewController : INSObject {
    fn view(&self) -> ShareId<UIView> {
        assert_main_thread!();
        unsafe {
            let view: *mut UIView = msg_send![self, view];
            ShareId::from_ptr(view)
        }
    }
}

pub struct UIViewController {
    _marker: NoSyncSend,
}

unsafe impl Message for UIViewController { }

impl INSObject for UIViewController {
    fn class() -> &'static Class {
        Class::get("UIViewController").unwrap()
    }

    // Redefine new to only allow constructing on the main thread
    fn new() -> Id<Self> {
        assert_main_thread!();
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        }
    }
}

impl IUIViewController for UIViewController { }
