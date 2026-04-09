use leptos::prelude::*;
use leptos_rerun::prelude::*;

const SAMPLE_MCAP_URL: &str = "https://media.githubusercontent.com/media/rerun-io/rerun/main/crates/store/re_data_loader/src/loader_mcap/tests/assets/foxglove_point_cloud.mcap";
const LOCAL_FOLLOW_SAMPLE_URL: &str = "http://127.0.0.1:4318/recording.mcap";
const OFFICIAL_TUTORIAL_URL: &str = "https://rerun.io/examples/robotics/mcap";

fn initial_state() -> (String, bool) {
    #[cfg(target_arch = "wasm32")]
    {
        let search = web_sys::window()
            .and_then(|window| window.location().search().ok())
            .unwrap_or_default();

        if let Ok(params) = web_sys::UrlSearchParams::new_with_str(&search) {
            let url = params
                .get("url")
                .unwrap_or_else(|| SAMPLE_MCAP_URL.to_string());
            let follow_if_http = params
                .get("follow_if_http")
                .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
                .unwrap_or(false);
            return (url, follow_if_http);
        }
    }

    (SAMPLE_MCAP_URL.to_string(), false)
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (initial_url, initial_follow_if_http) = initial_state();
    let source_url = RwSignal::new(initial_url);
    let follow_if_http = RwSignal::new(initial_follow_if_http);

    view! {
        <main
            style="min-height: 100vh; padding: 32px; box-sizing: border-box; background: radial-gradient(circle at top, #182338 0%, #0d121d 42%, #06080d 100%); color: #f4f7ff;"
        >
            <section style="width: min(1280px, 100%); margin: 0 auto; display: grid; gap: 18px;">
                <header style="display: grid; gap: 12px;">
                    <p style="margin: 0; font-size: 12px; letter-spacing: 0.18em; text-transform: uppercase; color: rgba(196, 211, 255, 0.62);">
                        "leptos-rerun"
                    </p>
                    <div style="display: flex; flex-wrap: wrap; align-items: end; justify-content: space-between; gap: 16px;">
                        <div>
                            <h1 style="margin: 0; font-size: clamp(34px, 4vw, 56px); line-height: 0.96; font-weight: 700;">
                                "MCAP Viewer"
                            </h1>
                            <p style="max-width: 760px; margin: 10px 0 0; font-size: 15px; line-height: 1.55; color: rgba(220, 228, 247, 0.78);">
                                "This loads a public MCAP file directly over HTTP as a normal remote file source."
                                " "
                                "To exercise "
                                <code>"follow_if_http"</code>
                                ", start "
                                <code>"scripts/serve_mcap.py"</code>
                                " with a local MCAP file and then click "
                                <code>"Use local follow sample"</code>
                                "."
                            </p>
                        </div>
                        <a
                            href=OFFICIAL_TUTORIAL_URL
                            target="_blank"
                            rel="noreferrer"
                            style="display: inline-flex; align-items: center; gap: 8px; padding: 11px 14px; border-radius: 999px; text-decoration: none; color: #f4f7ff; background: rgba(255, 255, 255, 0.08); border: 1px solid rgba(255, 255, 255, 0.12);"
                        >
                            "Official Rerun MCAP tutorial"
                        </a>
                    </div>
                </header>

                <div
                    style="display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 12px; padding: 16px; border-radius: 20px; background: rgba(10, 14, 22, 0.82); border: 1px solid rgba(255, 255, 255, 0.08);"
                >
                    <div style="display: grid; gap: 12px;">
                        <label style="display: grid; gap: 8px;">
                            <span style="font-size: 12px; letter-spacing: 0.08em; text-transform: uppercase; color: rgba(191, 203, 233, 0.62);">
                                "MCAP URL"
                            </span>
                            <input
                                type="url"
                                prop:value=move || source_url.get()
                                on:input=move |event| source_url.set(event_target_value(&event))
                                style="width: 100%; min-width: 0; padding: 14px 16px; border-radius: 14px; border: 1px solid rgba(255, 255, 255, 0.14); background: rgba(255, 255, 255, 0.05); color: #f4f7ff; font: inherit;"
                            />
                        </label>

                        <label style="display: inline-flex; align-items: center; gap: 10px; font-size: 14px; color: rgba(220, 228, 247, 0.86);">
                            <input
                                type="checkbox"
                                prop:checked=move || follow_if_http.get()
                                on:change=move |event| follow_if_http.set(event_target_checked(&event))
                            />
                            <span>
                                <code>"follow_if_http"</code>
                            </span>
                        </label>

                        <p style="margin: 0; font-size: 13px; line-height: 1.5; color: rgba(191, 203, 233, 0.7);">
                            "Helper server default: "
                            <code>{LOCAL_FOLLOW_SAMPLE_URL}</code>
                        </p>
                    </div>

                    <div style="display: grid; gap: 10px; align-self: end;">
                        <button
                            type="button"
                            on:click=move |_| {
                                source_url.set(SAMPLE_MCAP_URL.to_string());
                                follow_if_http.set(false);
                            }
                            style="padding: 14px 16px; border-radius: 14px; border: 1px solid rgba(109, 183, 255, 0.4); background: linear-gradient(135deg, rgba(24, 60, 117, 0.94) 0%, rgba(20, 132, 181, 0.92) 100%); color: #f4f7ff; font: inherit; cursor: pointer;"
                        >
                            "Reset public sample"
                        </button>

                        <span style="font-size: 12px; letter-spacing: 0.08em; text-transform: uppercase; color: rgba(191, 203, 233, 0.62);">
                            "Local follow demo"
                        </span>
                        <button
                            type="button"
                            on:click=move |_| {
                                source_url.set(LOCAL_FOLLOW_SAMPLE_URL.to_string());
                                follow_if_http.set(true);
                            }
                            style="padding: 14px 16px; border-radius: 14px; border: 1px solid rgba(171, 220, 138, 0.35); background: linear-gradient(135deg, rgba(67, 95, 37, 0.94) 0%, rgba(80, 137, 82, 0.92) 100%); color: #f4f7ff; font: inherit; cursor: pointer;"
                        >
                            "Use local follow sample"
                        </button>
                    </div>
                </div>

                <div
                    style="min-height: 70vh; border-radius: 24px; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.08); box-shadow: 0 18px 60px rgba(0, 0, 0, 0.36); background: linear-gradient(180deg, rgba(16, 20, 31, 0.96) 0%, rgba(10, 13, 20, 0.98) 100%);"
                >
                    <RerunViewer
                        style="width: 100%; height: 70vh; min-height: 560px;".to_string()
                        rrd=Signal::derive(move || source_url.get())
                        follow_if_http=Signal::derive(move || follow_if_http.get())
                        panel_state_overrides=[
                            (Panel::Top, PanelState::Hidden),
                            (Panel::Blueprint, PanelState::Hidden),
                            (Panel::Selection, PanelState::Hidden),
                            (Panel::Time, PanelState::Collapsed),
                        ]
                        hide_welcome_screen=true
                        theme=Theme::Dark
                    />
                </div>
            </section>
        </main>
    }
}
