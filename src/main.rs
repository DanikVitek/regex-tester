mod app;

use leptos::{mount_to_body, view};
use app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log::debug!("csr mode - mounting to body");

    mount_to_body(|| view!{ <App /> });
}
