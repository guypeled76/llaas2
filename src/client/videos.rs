use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn VideoPage() -> impl IntoView {
    let params = use_params_map();
    let video_id = move || params.read().get("video_id").unwrap_or_default();
    let language = move || params.read().get("language").unwrap_or_default();

    view! {
        <main>
            <h3>"Instant Subtitle Playback Sync"</h3>
            <p>{move || format!("Route: /videos/{}/{}", video_id(), language())}</p>
            <video id="myVideo" controls=true width="640">
                <source src={move || format!("/videos/{}.mp4", video_id())} type="video/mp4" />
                <track
                    id="subTrack"
                    src={move || format!("/videos/{}/{}/subtitles.vtt", video_id(), language())}
                    kind="subtitles"
                    srclang={language}
                    label={language}
                    default=true
                />
            </video>
            <div id="subtitle-timeline"></div>
        </main>
    }
}
