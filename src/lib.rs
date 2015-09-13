extern crate libc;
extern crate objc_foundation;

mod app;

pub use app::{application_main, IUIApplicationDelegate};

#[link(name = "UIKit", kind = "framework")]
extern { }
