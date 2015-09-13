#![allow(non_snake_case)]

extern crate objc;
extern crate objc_foundation;
extern crate objc_uikit;

mod app;

#[doc(hidden)]
pub mod extern_fns {
    pub use app::{
        RustApplicationDelegateCreate,
        RustApplicationDelegateDestroy,
        RustApplicationDelegateDidFinishLaunching,
    };
}

pub use app::{application_main, ApplicationDelegate};
