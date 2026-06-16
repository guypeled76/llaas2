use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main>
            <h1>"LLAAS"</h1>
            <p>"SPA client router is active."</p>
            <h3>"Open video route"</h3>
            <form id="video-form">
                <input id="video-id" placeholder="video id" required=true />
                <input id="video-lang" placeholder="lang (en, es, ...)" required=true />
                <button type="submit">"Open"</button>
            </form>
        </main>
    }
}

#[component]
pub fn VideoPage(video_id: String, language: String) -> impl IntoView {
    let route_text = format!("Route: /videos/{}/{}", video_id, language);
    view! {
        <main>
            <h3>"Instant Subtitle Playback Sync"</h3>
            <p>{route_text}</p>
            <video id="myVideo" controls=true width="640">
                <source src={format!("/videos/{}.mp4", video_id)} type="video/mp4" />
                <track
                    id="subTrack"
                    src={format!("/videos/{}/{}/subtitles.vtt", video_id, language)}
                    kind="subtitles"
                    srclang={language.clone()}
                    label={language}
                    default=true
                />
            </video>
            <div id="subtitle-timeline"></div>
        </main>
    }
}

pub fn spa_html() -> String {
    let _home_component = HomePage;
    let _video_component = VideoPage;

    let shell = r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>LLAAS</title>
  <style>
    body { font-family: sans-serif; margin: 40px; background: #121212; color: #fff; }
    #subtitle-timeline { margin-top: 20px; max-height: 300px; overflow-y: auto; padding: 10px; background: #1e1e1e; border-radius: 6px; }
    button { margin: 6px 0; padding: 10px; cursor: pointer; display: block; width: 100%; text-align: left; background: #2a2a2a; color: #fff; border: 1px solid #3a3a3a; border-radius: 4px; }
    button:hover { background: #3a3a3a; }
    video { border-radius: 6px; background: #000; max-width: 100%; height: auto; }
    input { margin-right: 8px; margin-bottom: 8px; padding: 8px; }
  </style>
</head>
<body>
  <div id="app"></div>
  <script>
    function routeMatch(pathname) {
      const match = pathname.match(/^\/videos\/([^/]+)\/([^/]+)$/);
      if (!match) return null;
      return { id: decodeURIComponent(match[1]), lang: decodeURIComponent(match[2]) };
    }

    function parseVttToButtons(vtt, container) {
      const blocks = vtt.split(/\r?\n\r?\n/);
      const cuePattern = /(\d{2}):(\d{2}):(\d{2})\.(\d{3})\s+-->\s+(\d{2}):(\d{2}):(\d{2})\.(\d{3})/;
      container.innerHTML = "";

      for (const block of blocks) {
        const lines = block.split(/\r?\n/).filter(Boolean);
        const timing = lines.find((line) => cuePattern.test(line));
        if (!timing) continue;

        const m = timing.match(cuePattern);
        if (!m) continue;

        const startSeconds =
          Number(m[1]) * 3600 +
          Number(m[2]) * 60 +
          Number(m[3]) +
          Number(m[4]) / 1000;

        const cueText = lines
          .filter((line) => line !== timing && !/^\d+$/.test(line))
          .join(" ")
          .trim();

        const labelMinutes = Math.floor(startSeconds / 60).toString().padStart(2, "0");
        const labelSeconds = Math.floor(startSeconds % 60).toString().padStart(2, "0");

        const btn = document.createElement("button");
        btn.textContent = `[${labelMinutes}:${labelSeconds}] "${cueText}"`;
        btn.addEventListener("click", () => {
          const video = document.getElementById("myVideo");
          if (!video) return;
          video.currentTime = startSeconds;
          video.play();
        });

        container.appendChild(btn);
      }
    }

    async function renderVideoPage(id, lang) {
      const app = document.getElementById("app");
      app.innerHTML = `
        <main>
          <h3>Instant Subtitle Playback Sync</h3>
          <p>Route: /videos/${encodeURIComponent(id)}/${encodeURIComponent(lang)}</p>
          <video id="myVideo" controls width="640">
            <source src="/videos/${encodeURIComponent(id)}.mp4" type="video/mp4">
            <track id="subTrack" src="/videos/${encodeURIComponent(id)}/${encodeURIComponent(lang)}/subtitles.vtt" kind="subtitles" srclang="${lang}" label="${lang}" default>
          </video>
          <div id="subtitle-timeline"></div>
        </main>
      `;

      try {
        const res = await fetch(`/videos/${encodeURIComponent(id)}/${encodeURIComponent(lang)}/subtitles.vtt`);
        if (!res.ok) throw new Error("Subtitle file not found");
        const vtt = await res.text();
        const timeline = document.getElementById("subtitle-timeline");
        if (timeline) parseVttToButtons(vtt, timeline);
      } catch (err) {
        const timeline = document.getElementById("subtitle-timeline");
        if (timeline) timeline.innerHTML = `<p>${String(err)}</p>`;
      }
    }

    function renderHomePage() {
      const app = document.getElementById("app");
      app.innerHTML = `
        <main>
          <h1>LLAAS</h1>
          <p>SPA client router is active.</p>
          <h3>Open video route</h3>
          <form id="video-form">
            <input id="video-id" placeholder="video id" required>
            <input id="video-lang" placeholder="lang (en, es, ...)" required>
            <button type="submit">Open</button>
          </form>
        </main>
      `;

      const form = document.getElementById("video-form");
      if (!form) return;

      form.addEventListener("submit", (event) => {
        event.preventDefault();
        const idInput = document.getElementById("video-id");
        const langInput = document.getElementById("video-lang");
        if (!idInput || !langInput) return;

        const id = idInput.value.trim();
        const lang = langInput.value.trim();
        if (!id || !lang) return;

        const nextPath = `/videos/${encodeURIComponent(id)}/${encodeURIComponent(lang)}`;
        history.pushState({}, "", nextPath);
        renderRouter();
      });
    }

    function renderRouter() {
      const match = routeMatch(window.location.pathname);
      if (match) {
        renderVideoPage(match.id, match.lang);
        return;
      }
      renderHomePage();
    }

    window.addEventListener("popstate", renderRouter);
    renderRouter();
  </script>
</body>
</html>"#;

    shell.to_string()
}
