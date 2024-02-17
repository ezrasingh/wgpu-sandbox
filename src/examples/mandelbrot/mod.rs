mod app;
use crate::examples::use_example;
use app::AppState;
use leptos::*;

#[component]
pub fn Mandelbrot() -> impl IntoView {
    let node = use_example::<AppState>();

    view! {
        <div node_ref=node/>
    }
}
