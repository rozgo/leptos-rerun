use leptos::prelude::*;
use leptos_meta::*;
use leptos_rerun::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

const SAMPLE_RRD_URL: &str = "https://app.rerun.io/version/0.31.2/examples/dna.rrd";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
                <style>
                    "html, body { margin: 0; width: 100%; height: 100%; background: #11131a; color: #f2f4ff; font-family: sans-serif; }"
                </style>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Leptos Rerun with Server"/>
        <Router>
            <main style="width: 100%; height: 100vh;">
                <Routes fallback=|| "This page couldn't be found">
                    <Route
                        path=path!("")
                        view=move || {
                            view! {
                                <RerunViewer
                                    style="width: 100%; height: 100%;".to_string()
                                    rrd=SAMPLE_RRD_URL
                                    panel_state_overrides=[(
                                        Panel::Blueprint,
                                        PanelState::Collapsed,
                                    )]
                                    hide_welcome_screen=true
                                    theme=Theme::Dark
                                />
                            }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
