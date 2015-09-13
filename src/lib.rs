extern crate libc;
extern crate core_graphics;
#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

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
