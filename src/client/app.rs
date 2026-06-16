use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::client::{
    homepage::HomePage,
    videos::VideoPage,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <main><p>"Page not found."</p></main> }>
                <Route path=path!("") view=HomePage />
                <Route path=path!("videos/:video_id/:language") view=VideoPage />
            </Routes>
        </Router>
    }
}