extern crate libc;
#[macro_use]
extern crate objc;
extern crate objc_foundation;

mod app;
mod view;

pub use app::{application_main, IUIApplicationDelegate};
pub use view::{IUIView, UIView};

#[link(name = "UIKit", kind = "framework")]
extern { }
