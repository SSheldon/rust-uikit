extern crate objc_id;
extern crate objc_foundation;
extern crate uikit;
extern crate uikit_impl;

use objc_id::ShareId;
use objc_foundation::INSObject;
use uikit::*;
use uikit_impl::ApplicationDelegate;

struct ExampleAppDelegate;

impl ApplicationDelegate for ExampleAppDelegate {
    fn root_view_controller(&self) -> ShareId<UIViewController> {
        let color = UIColor::from_rgba(0., 1., 0., 1.).share();
        let root_vc = UIViewController::new().share();
        root_vc.view().set_background_color(color);
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
