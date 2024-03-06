use crate::examples::triangle::Triangle;
use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="h-screen w-screen overflow-hidden">
            <Triangle/>
        </main>
    }
}
