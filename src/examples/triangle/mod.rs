mod app;
use crate::gpu::app::GpuApplication;
use app::AppState;
use leptos::*;
use std::sync::Arc;

#[allow(non_snake_case)]
#[component]
pub fn Triangle() -> impl IntoView {
    // ? create reference to DOM node
    let node = create_node_ref::<html::Div>();

    // ? use leptos signaling to encapsulate AppState
    let (app, _) = create_signal::<AppState>(Default::default());

    // ? getter closure for accesing div element
    let container = move || <web_sys::HtmlDivElement as Clone>::clone(&node.get().unwrap());

    // ? use thread to handle gpu runtime
    spawn_local(async move {
        // ? read runtime instance from read signal
        let mut runtime = app.get();

        let (window, event_loop) = runtime.setup(container());

        runtime.render(Arc::new(window), event_loop).await;
    });

    view! {
        <div node_ref=node/>
    }
}
