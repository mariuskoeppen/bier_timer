use crate::components::SimpleCounter;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! { <SimpleCounter initial_value=3/> }
}

