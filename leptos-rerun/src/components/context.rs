use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

#[cfg(target_arch = "wasm32")]
use std::sync::Mutex;

use leptos::prelude::*;

use crate::types::{Panel, PanelState, RerunViewerEvent};

#[cfg(target_arch = "wasm32")]
use crate::types::AssetOrigin;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

#[cfg(target_arch = "wasm32")]
use crate::bridge;

#[cfg_attr(
    not(all(not(feature = "ssr"), target_arch = "wasm32")),
    allow(dead_code)
)]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub(crate) struct StartupOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) manifest_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) render_backend: Option<crate::types::RenderBackend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) video_decoder: Option<crate::types::VideoDecoder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) theme: Option<crate::types::Theme>,
    pub(crate) hide_welcome_screen: bool,
    pub(crate) allow_fullscreen: bool,
}

#[cfg_attr(
    not(all(not(feature = "ssr"), target_arch = "wasm32")),
    allow(dead_code)
)]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub(crate) struct SourceRegistration {
    pub(crate) url: String,
    pub(crate) follow_if_http: bool,
}

#[cfg_attr(
    not(all(not(feature = "ssr"), target_arch = "wasm32")),
    allow(dead_code)
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub(crate) struct PanelRegistration {
    pub(crate) panel: Panel,
    pub(crate) state: PanelState,
}

#[cfg_attr(
    not(all(not(feature = "ssr"), target_arch = "wasm32")),
    allow(dead_code)
)]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub(crate) struct PlaybackOptions {
    pub(crate) autoplay: bool,
    pub(crate) loop_playback: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeline: Option<String>,
}

struct ViewerRuntime {
    ready: RwSignal<bool>,
    viewer_revision: RwSignal<u64>,
    last_event: RwSignal<Option<RerunViewerEvent>>,
    last_error: RwSignal<Option<String>>,
    starting: AtomicBool,
    lifecycle_nonce: AtomicU64,
    #[cfg(target_arch = "wasm32")]
    handle: Mutex<Option<send_wrapper::SendWrapper<JsValue>>>,
    #[cfg(target_arch = "wasm32")]
    event_callback: Mutex<Option<send_wrapper::SendWrapper<Closure<dyn FnMut(JsValue)>>>>,
}

impl ViewerRuntime {
    fn new() -> Self {
        Self {
            ready: RwSignal::new(false),
            viewer_revision: RwSignal::new(0),
            last_event: RwSignal::new(None),
            last_error: RwSignal::new(None),
            starting: AtomicBool::new(false),
            lifecycle_nonce: AtomicU64::new(0),
            #[cfg(target_arch = "wasm32")]
            handle: Mutex::new(None),
            #[cfg(target_arch = "wasm32")]
            event_callback: Mutex::new(None),
        }
    }
}

#[derive(Clone)]
pub struct RerunViewerContext {
    runtime: Arc<ViewerRuntime>,
}

impl RerunViewerContext {
    pub(crate) fn new() -> Self {
        Self {
            runtime: Arc::new(ViewerRuntime::new()),
        }
    }

    pub fn ready(&self) -> bool {
        self.runtime.ready.get()
    }

    pub fn ready_signal(&self) -> ReadSignal<bool> {
        self.runtime.ready.read_only()
    }

    #[cfg_attr(
        not(all(not(feature = "ssr"), target_arch = "wasm32")),
        allow(dead_code)
    )]
    pub(crate) fn viewer_revision_signal(&self) -> ReadSignal<u64> {
        self.runtime.viewer_revision.read_only()
    }

    pub fn last_event(&self) -> Option<RerunViewerEvent> {
        self.runtime.last_event.get()
    }

    pub fn last_event_signal(&self) -> ReadSignal<Option<RerunViewerEvent>> {
        self.runtime.last_event.read_only()
    }

    pub fn last_error(&self) -> Option<String> {
        self.runtime.last_error.get()
    }

    pub fn last_error_signal(&self) -> ReadSignal<Option<String>> {
        self.runtime.last_error.read_only()
    }

    pub fn open(&self, url: impl AsRef<str>, follow_if_http: bool) {
        #[cfg(target_arch = "wasm32")]
        self.with_bridge(move |bridge_handle| {
            bridge::open_source(bridge_handle, url.as_ref(), follow_if_http)
        });
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (url, follow_if_http);
    }

    pub fn close(&self, url: impl AsRef<str>) {
        #[cfg(target_arch = "wasm32")]
        self.with_bridge(move |bridge_handle| bridge::close_source(bridge_handle, url.as_ref()));
        #[cfg(not(target_arch = "wasm32"))]
        let _ = url;
    }

    pub fn set_active_recording_id(&self, recording_id: impl AsRef<str>) {
        #[cfg(target_arch = "wasm32")]
        self.with_bridge(move |bridge_handle| {
            bridge::set_active_recording_id(bridge_handle, recording_id.as_ref())
        });
        #[cfg(not(target_arch = "wasm32"))]
        let _ = recording_id;
    }

    pub fn set_playing(&self, recording_id: impl AsRef<str>, value: bool) {
        #[cfg(target_arch = "wasm32")]
        self.with_bridge(move |bridge_handle| {
            bridge::set_playing(bridge_handle, recording_id.as_ref(), value)
        });
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (recording_id, value);
    }

    pub fn set_current_time(
        &self,
        recording_id: impl AsRef<str>,
        timeline: impl AsRef<str>,
        time: f64,
    ) {
        #[cfg(target_arch = "wasm32")]
        self.with_bridge(move |bridge_handle| {
            bridge::set_current_time(
                bridge_handle,
                recording_id.as_ref(),
                timeline.as_ref(),
                time,
            )
        });
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (recording_id, timeline, time);
    }

    pub fn set_active_timeline(&self, recording_id: impl AsRef<str>, timeline: impl AsRef<str>) {
        #[cfg(target_arch = "wasm32")]
        self.with_bridge(move |bridge_handle| {
            bridge::set_active_timeline(bridge_handle, recording_id.as_ref(), timeline.as_ref())
        });
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (recording_id, timeline);
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn start_viewer(
        &self,
        parent: web_sys::HtmlElement,
        startup_options: StartupOptions,
        asset_origin: AssetOrigin,
        on_event: Option<Callback<RerunViewerEvent>>,
    ) {
        if self.runtime.starting.load(Ordering::Acquire)
            || self
                .runtime
                .handle
                .lock()
                .expect("viewer handle mutex poisoned")
                .is_some()
        {
            return;
        }

        self.runtime.starting.store(true, Ordering::Release);
        self.runtime.ready.set(false);
        self.runtime.last_error.set(None);

        let nonce = self
            .runtime
            .lifecycle_nonce
            .fetch_add(1, Ordering::AcqRel)
            .wrapping_add(1);

        let module_url = asset_origin.module_url();
        let context = self.clone();
        let event_context = self.clone();

        let callback = Closure::wrap(Box::new(move |value: JsValue| {
            match serde_wasm_bindgen::from_value::<RerunViewerEvent>(value) {
                Ok(event) => {
                    if matches!(event, RerunViewerEvent::Ready) {
                        event_context.runtime.ready.set(true);
                    }
                    event_context.runtime.last_event.set(Some(event.clone()));
                    if let Some(on_event) = on_event.as_ref() {
                        on_event.run(event);
                    }
                }
                Err(error) => {
                    event_context
                        .record_error(format!("failed to deserialize viewer event: {error}"));
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        let event_function: js_sys::Function = callback
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone();

        wasm_bindgen_futures::spawn_local(async move {
            let startup_value = match serde_wasm_bindgen::to_value(&startup_options) {
                Ok(value) => value,
                Err(error) => {
                    context.runtime.starting.store(false, Ordering::Release);
                    context.record_error(format!("failed to serialize startup options: {error}"));
                    return;
                }
            };

            match bridge::create_viewer(parent, &module_url, startup_value, &event_function).await {
                Ok(bridge_handle) => {
                    context.runtime.starting.store(false, Ordering::Release);

                    if context.runtime.lifecycle_nonce.load(Ordering::Acquire) != nonce {
                        let _ = bridge::destroy_viewer(&bridge_handle);
                        return;
                    }

                    *context
                        .runtime
                        .handle
                        .lock()
                        .expect("viewer handle mutex poisoned") =
                        Some(send_wrapper::SendWrapper::new(bridge_handle));
                    *context
                        .runtime
                        .event_callback
                        .lock()
                        .expect("event callback mutex poisoned") =
                        Some(send_wrapper::SendWrapper::new(callback));
                    context
                        .runtime
                        .viewer_revision
                        .update(|revision| *revision += 1);
                }
                Err(error) => {
                    context.runtime.starting.store(false, Ordering::Release);
                    context.record_error(js_value_to_string(error));
                }
            }
        });
    }

    pub(crate) fn destroy_viewer(&self) {
        self.runtime.lifecycle_nonce.fetch_add(1, Ordering::AcqRel);
        self.runtime.starting.store(false, Ordering::Release);
        self.runtime.ready.set(false);
        self.runtime.viewer_revision.set(0);

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(bridge_handle) = self
                .runtime
                .handle
                .lock()
                .expect("viewer handle mutex poisoned")
                .take()
                && let Err(error) = bridge::destroy_viewer(&bridge_handle)
            {
                self.record_error(js_value_to_string(error));
            }
            self.runtime
                .event_callback
                .lock()
                .expect("event callback mutex poisoned")
                .take();
        }
    }

    #[cfg_attr(
        not(all(not(feature = "ssr"), target_arch = "wasm32")),
        allow(dead_code)
    )]
    pub(crate) fn sync_sources(&self, _urls: &[String], follow_if_http: bool) {
        #[cfg(target_arch = "wasm32")]
        {
            let sources: Vec<SourceRegistration> = _urls
                .iter()
                .filter_map(|url| {
                    let url = url.trim();
                    if url.is_empty() {
                        None
                    } else {
                        Some(SourceRegistration {
                            url: url.to_string(),
                            follow_if_http,
                        })
                    }
                })
                .collect();

            self.with_bridge(move |bridge_handle| {
                let sources_value = serde_wasm_bindgen::to_value(&sources)
                    .map_err(|error| JsValue::from_str(&error.to_string()))?;
                bridge::sync_sources(bridge_handle, sources_value)
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (_urls, follow_if_http);
    }

    #[cfg_attr(
        not(all(not(feature = "ssr"), target_arch = "wasm32")),
        allow(dead_code)
    )]
    pub(crate) fn sync_panel_overrides(&self, _overrides: &[PanelRegistration]) {
        #[cfg(target_arch = "wasm32")]
        {
            let panels = _overrides.to_vec();
            let enabled = !_overrides.is_empty();
            self.with_bridge(move |bridge_handle| {
                let panels_value = serde_wasm_bindgen::to_value(&panels)
                    .map_err(|error| JsValue::from_str(&error.to_string()))?;
                bridge::sync_panel_overrides(bridge_handle, panels_value, enabled)
            });
        }
    }

    #[cfg_attr(
        not(all(not(feature = "ssr"), target_arch = "wasm32")),
        allow(dead_code)
    )]
    pub(crate) fn sync_playback_options(&self, _options: &PlaybackOptions) {
        #[cfg(target_arch = "wasm32")]
        {
            let options = _options.clone();
            self.with_bridge(move |bridge_handle| {
                let options_value = serde_wasm_bindgen::to_value(&options)
                    .map_err(|error| JsValue::from_str(&error.to_string()))?;
                bridge::sync_playback_options(bridge_handle, options_value)
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = _options;
    }

    #[cfg(target_arch = "wasm32")]
    fn with_bridge(&self, f: impl FnOnce(&JsValue) -> Result<(), JsValue>) {
        let handle = self
            .runtime
            .handle
            .lock()
            .expect("viewer handle mutex poisoned");
        let Some(bridge_handle) = handle.as_ref() else {
            return;
        };

        if let Err(error) = f(bridge_handle) {
            self.record_error(js_value_to_string(error));
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn record_error(&self, message: String) {
        self.runtime.last_error.set(Some(message.clone()));
        leptos::logging::error!("{message}");
    }
}

pub fn provide_rerun_viewer_context() -> RerunViewerContext {
    let context = RerunViewerContext::new();
    provide_context(context.clone());
    context
}

pub fn use_rerun_viewer_context() -> Option<RerunViewerContext> {
    use_context::<RerunViewerContext>()
}

#[cfg(target_arch = "wasm32")]
fn js_value_to_string(error: JsValue) -> String {
    error
        .as_string()
        .or_else(|| {
            js_sys::JSON::stringify(&error)
                .ok()
                .and_then(|value| value.as_string())
        })
        .unwrap_or_else(|| format!("{error:?}"))
}
