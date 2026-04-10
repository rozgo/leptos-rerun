use leptos::{html::Div, prelude::*};

use crate::{
    components::{RerunViewerContext, provide_rerun_viewer_context},
    types::{
        AssetOrigin, PanelStateOverridesProp, RenderBackend, RerunViewerEvent, RrdProp, Theme,
        VideoDecoder,
    },
};

#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
use crate::components::context::{PanelRegistration, StartupOptions};
#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
use wasm_bindgen::{JsCast, closure::Closure};

fn normalize_container_style(style: &str) -> String {
    if style.trim().is_empty() {
        "position: relative; width: 100%; min-height: 360px;".to_string()
    } else if style.contains("position:") {
        style.to_string()
    } else {
        format!("position: relative; {style}")
    }
}

#[component]
pub fn RerunViewer(
    #[prop(optional)] class: String,
    #[prop(optional)] style: String,
    #[prop(optional, default = NodeRef::new())] node_ref: NodeRef<Div>,
    #[prop(optional, into, default = RrdProp::default())] rrd: RrdProp,
    #[prop(optional, into, default = PanelStateOverridesProp::default())]
    panel_state_overrides: PanelStateOverridesProp,
    #[prop(optional, default = AssetOrigin::default())] asset_origin: AssetOrigin,
    #[prop(optional, into)] theme: Signal<Option<Theme>>,
    #[prop(optional, into)] render_backend: Signal<Option<RenderBackend>>,
    #[prop(optional, into)] video_decoder: Signal<Option<VideoDecoder>>,
    #[prop(optional, into, default = false.into())] hide_welcome_screen: Signal<bool>,
    #[prop(optional, into)] manifest_url: Signal<Option<String>>,
    #[prop(optional, into, default = false.into())] allow_fullscreen: Signal<bool>,
    #[prop(optional, into, default = false.into())] follow_if_http: Signal<bool>,
    #[prop(optional)] on_event: Option<Callback<RerunViewerEvent>>,
) -> impl IntoView {
    let context = provide_rerun_viewer_context();

    #[cfg(not(target_arch = "wasm32"))]
    let _ = (
        &rrd,
        &panel_state_overrides,
        &asset_origin,
        &theme,
        &render_backend,
        &video_decoder,
        &hide_welcome_screen,
        &manifest_url,
        &allow_fullscreen,
        &follow_if_http,
        &on_event,
    );

    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    let last_started_options = RwSignal::new(None::<(AssetOrigin, StartupOptions)>);
    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    let pending_start_nonce = RwSignal::new(0_u64);

    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    Effect::new({
        let context = context.clone();
        let node_ref = node_ref.clone();
        let asset_origin = asset_origin.clone();
        let theme = theme.clone();
        let render_backend = render_backend.clone();
        let video_decoder = video_decoder.clone();
        let hide_welcome_screen = hide_welcome_screen.clone();
        let manifest_url = manifest_url.clone();
        let allow_fullscreen = allow_fullscreen.clone();
        let on_event = on_event.clone();
        let last_started_options = last_started_options;
        let pending_start_nonce = pending_start_nonce;

        move |_| {
            if node_ref.get().is_none() {
                return;
            }

            let startup_options = StartupOptions {
                manifest_url: manifest_url.get(),
                render_backend: render_backend.get(),
                video_decoder: video_decoder.get(),
                theme: theme.get(),
                hide_welcome_screen: hide_welcome_screen.get(),
                allow_fullscreen: allow_fullscreen.get(),
            };
            let next_options = (asset_origin.clone(), startup_options.clone());
            let should_restart =
                last_started_options.with(|previous| previous.as_ref() != Some(&next_options));

            if !should_restart {
                return;
            }

            last_started_options.set(Some(next_options));
            context.destroy_viewer();
            pending_start_nonce.update(|nonce| *nonce = nonce.wrapping_add(1));
            schedule_viewer_start(
                node_ref.clone(),
                pending_start_nonce,
                context.clone(),
                startup_options,
                asset_origin.clone(),
                on_event.clone(),
            );
        }
    });

    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    Effect::new({
        let context = context.clone();
        let viewer_revision = context.viewer_revision_signal();
        let rrd = rrd.clone();
        let follow_if_http = follow_if_http.clone();

        move |_| {
            if viewer_revision.get() == 0 {
                return;
            }

            let sources = rrd.get();
            context.sync_sources(sources.as_slice(), follow_if_http.get());
        }
    });

    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    Effect::new({
        let context = context.clone();
        let viewer_revision = context.viewer_revision_signal();
        let panel_state_overrides = panel_state_overrides.clone();

        move |_| {
            if viewer_revision.get() == 0 {
                return;
            }

            let overrides = panel_state_overrides.get();
            let registrations: Vec<PanelRegistration> = overrides
                .iter()
                .map(|(panel, state)| PanelRegistration { panel, state })
                .collect();
            context.sync_panel_overrides(&registrations);
        }
    });

    on_cleanup(move || {
        context.destroy_viewer();
    });

    let style = normalize_container_style(&style);

    view! {
        <div class=class style=style node_ref=node_ref></div>
    }
}

#[allow(dead_code)]
fn _assert_context_is_cloneable(_context: RerunViewerContext) {}

#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
fn schedule_viewer_start(
    node_ref: NodeRef<Div>,
    pending_start_nonce: RwSignal<u64>,
    context: RerunViewerContext,
    startup_options: StartupOptions,
    asset_origin: AssetOrigin,
    on_event: Option<Callback<RerunViewerEvent>>,
) {
    let expected_nonce = pending_start_nonce.get_untracked();
    let callback = Closure::once_into_js(move || {
        if pending_start_nonce
            .try_get_untracked()
            .is_none_or(|nonce| nonce != expected_nonce)
        {
            return;
        }

        let Some(div) = node_ref.get() else {
            return;
        };
        let element: web_sys::HtmlElement = div.into();
        if !element.is_connected() {
            return;
        }

        context.start_viewer(element, startup_options, asset_origin, on_event);
    });

    if let Some(window) = web_sys::window() {
        let _ = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), 0);
    }
}
