use leptos::*;
use leptos_example::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
