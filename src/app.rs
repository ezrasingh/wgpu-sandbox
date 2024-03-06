use crate::examples;
use leptos::*;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="flex h-full w-full justify-center items-center">
                <h1>WGPU Sandbox</h1>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="flex justify-between fixed top-0 left-0 right-0 h-8 z-50 bg-white">
                <span class="mx-4"><A class="link" href="" exact=true>WGPU Sandbox</A></span>
                <ul class="flex">
                    <li class="mx-4"><A class="link" href="triangle" exact=true>Triangle</A></li>
                    <li class="mx-4"><A class="link" href="mandelbrot" exact=true>Mandelbrot</A></li>
                </ul>
            </nav>
            <main class="h-screen w-screen overflow-hidden bg-black text-white pt-8">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/triangle" view=examples::triangle::Triangle/>
                    <Route path="/mandelbrot" view=examples::mandelbrot::Mandelbrot/>
                </Routes>
            </main>
        </Router>
    }
}
