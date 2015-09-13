use std::mem;

use objc::Message;
use objc::runtime::{BOOL, NO, YES, Class};
use objc_foundation::INSObject;
use objc_uikit::{IUIApplicationDelegate, self};

pub trait ApplicationDelegate {
    fn did_finish_launching(&self) -> bool;
}

extern {
    fn RustApplicationDelegateClass() -> *mut Class;
}

struct ApplicationDelegateObj {
    _private: (),
}

unsafe impl Message for ApplicationDelegateObj { }

impl INSObject for ApplicationDelegateObj {
    fn class() -> &'static Class {
        unsafe {
            let cls = RustApplicationDelegateClass();
            assert!(!cls.is_null());
            &*cls
        }
    }
}

impl IUIApplicationDelegate for ApplicationDelegateObj { }

static mut APP_DELEGATE_PTR: Option<*mut ApplicationDelegate> = None;

pub fn application_main<T: ApplicationDelegate>(delegate: T) -> ! {
    let delegate: Box<ApplicationDelegate> = Box::new(delegate);
    unsafe {
        APP_DELEGATE_PTR = Some(mem::transmute(delegate));
    }

    objc_uikit::application_main::<ApplicationDelegateObj>();
}

#[no_mangle]
pub unsafe extern fn RustApplicationDelegateCreate(out: *mut *mut ApplicationDelegate) -> BOOL {
    if let Some(delegate) = APP_DELEGATE_PTR.take() {
        *out = delegate;
        YES
    } else {
        NO
    }
}

#[no_mangle]
pub unsafe extern fn RustApplicationDelegateDestroy(obj: *mut ApplicationDelegate) {
    let delegate: Box<ApplicationDelegate> = mem::transmute(obj);
    drop(delegate);
}

#[no_mangle]
pub unsafe extern fn RustApplicationDelegateDidFinishLaunching(obj: *mut ApplicationDelegate) -> BOOL {
    if (&*obj).did_finish_launching() {
        YES
    } else {
        NO
    }
}
