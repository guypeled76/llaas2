use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleItem {
    pub text: String,
}

#[server]
pub async fn get_subtitle_items() -> Result<Vec<SubtitleItem>, ServerFnError> {
    Ok(vec![
        SubtitleItem {
            text: "Welcome to the show".to_string(),
        },
        SubtitleItem {
            text: "Let's get started".to_string(),
        },
        SubtitleItem {
            text: "Here is the first topic".to_string(),
        },
        SubtitleItem {
            text: "And now the conclusion".to_string(),
        },
        SubtitleItem {
            text: "Thanks for watching".to_string(),
        },
    ])
}

#[component]
pub fn VideoPage() -> impl IntoView {
    let params = use_params_map();
    let video_id = move || params.read().get("video_id").unwrap_or_default();
    let language = move || params.read().get("language").unwrap_or_default();

    let subtitles = OnceResource::new(get_subtitle_items());

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
            <div id="subtitle-timeline">
                <Suspense fallback=|| view! { <p>"Loading subtitles..."</p> }>
                    {move || Suspend::new(async move {
                        subtitles.await.map(|items| {
                            items
                                .into_iter()
                                .map(|item| view! { <button>{item.text}</button> })
                                .collect_view()
                        })
                    })}
                </Suspense>
            </div>
        </main>
    }
}
