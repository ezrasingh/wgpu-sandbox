use crate::examples::mandelbrot::Mandelbrot;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="h-screen w-screen overflow-hidden">
            <Mandelbrot/>
        </main>
    }
}
