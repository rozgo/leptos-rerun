use leptos::prelude::*;
use leptos_rerun::prelude::*;

const SAMPLE_RRD_URL: &str = "https://app.rerun.io/version/0.31.2/examples/dna.rrd";

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main style="width: 100%; height: 100vh; background: #11131a;">
            <RerunViewer
                style="width: 100%; height: 100%;".to_string()
                rrd=SAMPLE_RRD_URL
                panel_state_overrides=[(Panel::Blueprint, PanelState::Collapsed)]
                hide_welcome_screen=true
                theme=Theme::Dark
            />
        </main>
    }
}
