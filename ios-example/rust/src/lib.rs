extern crate core_graphics;
extern crate objc_id;
extern crate objc_foundation;
extern crate uikit;
extern crate uikit_impl;

use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use objc_id::ShareId;
use objc_foundation::INSObject;
use uikit::*;
use uikit_impl::ApplicationDelegate;

const LOGO: &'static [u8] = include_bytes!("../rust_logo.png");

struct ExampleAppDelegate;

impl ApplicationDelegate for ExampleAppDelegate {
    fn root_view_controller(&self) -> ShareId<UIViewController> {
        let green = UIColor::from_rgba(0., 1., 0., 1.).share();
        let blue = UIColor::from_rgba(0., 0., 1., 1.).share();

        let image = UIImage::with_bytes(LOGO).share();

        let root_vc = UIViewController::new().share();
        root_vc.view().set_background_color(green);

        let frame = CGRect {
            origin: CGPoint { x: 100., y: 100. },
            size: CGSize { width: 100., height: 100. },
        };
        let subview = UIImageView::with_image(image).share();
        subview.set_frame(frame);
        subview.set_background_color(blue);
        root_vc.view().add_subview(subview);

        root_vc
    }

    fn did_finish_launching(&self) -> bool {
        true
    }
}

#[no_mangle]
pub extern fn run_app() {
    uikit_impl::application_main(ExampleAppDelegate);
}
