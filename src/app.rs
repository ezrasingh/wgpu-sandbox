use crate::display::AppState;
use crate::gpu::GpuApplication;
use leptos::*;
use std::sync::Arc;
use web_sys::HtmlDivElement;

#[component]
pub fn App() -> impl IntoView {
    let node = create_node_ref::<html::Div>();
    let (app, _) = create_signal::<AppState>(Default::default());
    let container = move || <HtmlDivElement as Clone>::clone(&node.get().unwrap());

    spawn_local(async move {
        let runtime = app.get();
        let (window, event_loop) = runtime.setup(container());
        runtime.render(Arc::new(window), event_loop).await;
    });

    view! {
        <div class="flex" node_ref=node/>
    }
}
