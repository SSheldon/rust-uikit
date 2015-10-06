extern crate libc;
extern crate core_graphics;
#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

macro_rules! assert_main_thread {
    () => (
        assert!($crate::is_main_thread())
    );
}

mod app;
mod color;
mod view;
mod view_controller;

pub use app::{application_main, IUIApplicationDelegate};
pub use color::UIColor;
pub use view::{IUIView, UIView};
pub use view_controller::{IUIViewController, UIViewController};

#[link(name = "UIKit", kind = "framework")]
extern { }

fn is_main_thread() -> bool {
    use objc::runtime::{Class, BOOL, NO};

    let cls = Class::get("NSThread").unwrap();
    let result: BOOL = unsafe { msg_send![cls, isMainThread] };
    result != NO
}

struct NoSyncSend {
    _marker: ::std::marker::PhantomData<*mut ::libc::c_void>,
}
