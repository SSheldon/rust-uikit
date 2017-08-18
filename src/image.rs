use std::path::Path;
use objc::Message;
use objc::runtime::Class;
use objc_id::Id;
use objc_foundation::{INSObject, INSString, NSString, INSData, NSData};

pub struct UIImage {
    _private: (),
}

unsafe impl Message for UIImage { }

impl INSObject for UIImage {
    fn class() -> &'static Class {
        Class::get("UIImage").unwrap()
    }
}

impl UIImage {
    pub fn from_path(location: &Path)
            -> Id<Self> {
        let cls = Self::class();
        let p = NSString::from_str(&location.to_string_lossy());
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, imageWithContentsOfFile:p];
            Id::from_retained_ptr(obj)
        }
    }
    pub fn with_bytes(bytes: &[u8])
            -> Id<Self> {
        let cls = Self::class();
        let data = NSData::with_bytes(bytes);
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithData:data];
            Id::from_retained_ptr(obj)
        }
    }
    pub fn from_vec(vec_data: Vec<u8>)
            -> Id<Self> {
        let cls = Self::class();
        let data = NSData::from_vec(vec_data);
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithData:data];
            Id::from_retained_ptr(obj)
        }
    }
}
