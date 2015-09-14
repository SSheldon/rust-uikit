use core_graphics::geometry::CGRect;
use objc::Message;
use objc::runtime::Class;
use objc_id::{Id, ShareId};
use objc_foundation::INSObject;

use UIColor;

pub trait IUIView : INSObject {
    fn with_frame(frame: CGRect) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithFrame:frame];
            Id::from_retained_ptr(obj)
        }
    }

    fn frame(&self) -> CGRect {
        unsafe {
            msg_send![self, frame]
        }
    }

    fn set_frame(&self, frame: CGRect) {
        unsafe {
            msg_send![self, setFrame:frame]
        }
    }

    fn add_subview<T: IUIView>(&self, view: ShareId<T>) {
        unsafe {
            msg_send![self, addSubview:&*view]
        }
    }

    fn remove_from_superview(&self) {
        unsafe {
            msg_send![self, removeFromSuperview]
        }
    }

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
