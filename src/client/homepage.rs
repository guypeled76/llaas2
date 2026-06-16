use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main>
            <h1>"LLAAS"</h1>
            <p>"Leptos router is active."</p>
            <h3>"Open video route"</h3>
            <form id="video-form">
                <input id="video-id" placeholder="video id" required=true />
                <input id="video-lang" placeholder="lang (en, es, ...)" required=true />
                <button type="submit">"Open"</button>
            </form>
        </main>
    }
}
