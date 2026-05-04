use leptos::prelude::*;
use leptos_rerun::prelude::*;

struct GalleryExample {
    title: &'static str,
    category: &'static str,
    url: &'static str,
}

const EXAMPLES: [GalleryExample; 6] = [
    GalleryExample {
        title: "DNA",
        category: "3D points",
        url: "https://app.rerun.io/version/0.31.3/examples/dna.rrd",
    },
    GalleryExample {
        title: "Plots",
        category: "Charts",
        url: "https://app.rerun.io/version/0.31.3/examples/plots.rrd",
    },
    GalleryExample {
        title: "Graphs",
        category: "Node-link views",
        url: "https://app.rerun.io/version/0.31.3/examples/graphs.rrd",
    },
    GalleryExample {
        title: "Tracking",
        category: "Detection video",
        url: "https://app.rerun.io/version/0.31.3/examples/detect_and_track_objects.rrd",
    },
    GalleryExample {
        title: "Raw Mesh",
        category: "Geometry",
        url: "https://app.rerun.io/version/0.31.3/examples/raw_mesh.rrd",
    },
    GalleryExample {
        title: "ARKit Scenes",
        category: "Spatial capture",
        url: "https://app.rerun.io/version/0.31.3/examples/arkit_scenes.rrd",
    },
];

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main
            style="min-height: 100vh; padding: 40px 32px 48px; background: radial-gradient(circle at top, #1a2233 0%, #0d121d 48%, #06080d 100%); color: #f4f7ff;"
        >
            <section style="width: min(1480px, 100%); margin: 0 auto;">
                <header style="display: flex; align-items: end; justify-content: space-between; gap: 24px; margin-bottom: 24px;">
                    <div>
                        <p style="margin: 0 0 10px; font-size: 12px; letter-spacing: 0.18em; text-transform: uppercase; color: rgba(196, 211, 255, 0.62);">
                            "leptos-rerun"
                        </p>
                        <h1 style="margin: 0; font-size: clamp(34px, 4vw, 56px); line-height: 0.96; font-weight: 700;">
                            "Embedded Rerun Gallery"
                        </h1>
                    </div>
                    <p style="max-width: 440px; margin: 0; font-size: 15px; line-height: 1.5; color: rgba(220, 228, 247, 0.78);">
                        "Six public Rerun recordings, each embedded as a native viewer card with compact controls."
                    </p>
                </header>

                <div
                    style="display: grid; grid-template-columns: repeat(auto-fit, minmax(380px, 1fr)); gap: 20px;"
                >
                    <For
                        each=move || EXAMPLES.into_iter().enumerate()
                        key=|(index, _)| *index
                        children=move |(_, example)| {
                            view! {
                                <article
                                    style="display: grid; grid-template-rows: auto auto; border-radius: 24px; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.08); box-shadow: 0 18px 60px rgba(0, 0, 0, 0.36); background: linear-gradient(180deg, rgba(16, 20, 31, 0.96) 0%, rgba(10, 13, 20, 0.98) 100%);"
                                >
                                    <div style="display: flex; align-items: center; justify-content: space-between; gap: 16px; padding: 14px 16px 12px; border-bottom: 1px solid rgba(255, 255, 255, 0.06);">
                                        <div>
                                            <h2 style="margin: 0; font-size: 16px; font-weight: 650; line-height: 1.1;">
                                                {example.title}
                                            </h2>
                                            <p style="margin: 4px 0 0; font-size: 12px; letter-spacing: 0.08em; text-transform: uppercase; color: rgba(191, 203, 233, 0.62);">
                                                {example.category}
                                            </p>
                                        </div>
                                        <div style="width: 10px; height: 10px; border-radius: 999px; background: linear-gradient(135deg, #9ae6b4 0%, #63b3ed 100%); box-shadow: 0 0 18px rgba(99, 179, 237, 0.4); flex: 0 0 auto;"></div>
                                    </div>

                                    <div style="padding: 0; aspect-ratio: 16 / 10; min-height: 286px;">
                                        <RerunViewer
                                            style="width: 100%; height: 100%;".to_string()
                                            rrd=example.url
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
                                </article>
                            }
                        }
                    />
                </div>
            </section>
        </main>
    }
}
