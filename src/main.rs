mod app;
mod model;

use leptos::logging;
use leptos::prelude::mount_to_body;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");
    mount_to_body(App);
}
