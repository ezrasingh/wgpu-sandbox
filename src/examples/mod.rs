pub mod mandelbrot;
pub mod triangle;

use leptos::*;
use std::sync::Arc;

use crate::gpu::app::GpuApplication;

fn use_example<S>() -> NodeRef<html::Div>
where
    S: Clone
        + Copy
        + Default
        + encase::ShaderType
        + encase::internal::WriteInto
        + GpuApplication
        + 'static,
{
    let node = create_node_ref::<html::Div>();
    let (app, _) = create_signal::<S>(Default::default());
    let container = move || <web_sys::HtmlDivElement as Clone>::clone(&node.get().unwrap());

    spawn_local(async move {
        let mut runtime = app.get();
        let (window, event_loop) = runtime.setup(container());
        runtime.render(Arc::new(window), event_loop).await;
    });

    node
}
