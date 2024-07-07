use crate::components::snake::Snake;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Home/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="h-screen w-screen flex-col justify-between text-center text-neutral-100" >
            <div class="text-4xl my-5">
                "Welcome to my recreation of snake"
            </div>
            <Snake/>
        </div>
    }
}
