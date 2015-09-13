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
mod window;

pub use app::{application_main, IUIApplicationDelegate};
pub use color::UIColor;
pub use view::{IUIView, UIView};
pub use view_controller::{IUIViewController, UIViewController};
pub use window::UIWindow;

#[link(name = "UIKit", kind = "framework")]
extern { }
