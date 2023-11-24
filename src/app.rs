use crate::pages::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                " | "
                <A href="/about">"About"</A>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=move || view! { <Home/> }/>
                    <Route path="/about" view=move || view! { <About/> }/>
                </Routes>
            </main>
        </Router>
    }
}

