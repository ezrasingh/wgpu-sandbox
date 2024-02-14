use crate::display::AppState;
use crate::gpu::GpuApplication;
use leptos::*;
use web_sys::HtmlCanvasElement;
use winit::window::WindowBuilder;

#[component]
pub fn App() -> impl IntoView {
    let node = create_node_ref::<html::Canvas>();
    let (app, _) = create_signal::<AppState>(Default::default());
    let canvas = move || <HtmlCanvasElement as Clone>::clone(&node.get_untracked().unwrap());
    let builder = move || <WindowBuilder as Clone>::clone(&app.get().setup(canvas()));

    spawn_local(async move {
        app.get().render(builder()).await;
    });

    view! {
        <canvas node_ref=node/>
    }
}
