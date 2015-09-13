use core_graphics::base::CGFloat;
use objc::Message;
use objc::runtime::Class;
use objc_id::Id;
use objc_foundation::INSObject;

pub struct UIColor {
    _private: (),
}

unsafe impl Message for UIColor { }

impl INSObject for UIColor {
    fn class() -> &'static Class {
        Class::get("UIColor").unwrap()
    }
}

impl UIColor {
    pub fn from_rgba(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat)
            -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithRed:red
                                                      green:green
                                                       blue:blue
                                                      alpha:alpha];
            Id::from_retained_ptr(obj)
        }
    }
}
