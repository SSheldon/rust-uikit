use core_graphics::geometry::CGRect;
use objc::Message;
use objc::runtime::Class;
use objc_id::{Id, ShareId};
use objc_foundation::INSObject;

use {NoSyncSend, IUIView, UIImage};

impl IUIView for UIImageView { }

pub struct UIImageView {
    _marker: NoSyncSend,
}

unsafe impl Message for UIImageView { }

impl INSObject for UIImageView {
    fn class() -> &'static Class {
        Class::get("UIImageView").unwrap()
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


impl UIImageView {
    pub fn with_image(image: ShareId<UIImage>) -> Id<Self> {
        assert_main_thread!();
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithImage:image];
            Id::from_retained_ptr(obj)
        }
    }
}
