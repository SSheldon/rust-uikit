extern crate libc;
extern crate core_graphics;
#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

mod app;
mod color;
mod view;

pub use app::{application_main, IUIApplicationDelegate};
pub use color::UIColor;
pub use view::{IUIView, UIView};

#[link(name = "UIKit", kind = "framework")]
extern { }
