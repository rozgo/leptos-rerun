#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
const __leptosRerunViewerModules = new Map();

async function loadWebViewer(moduleUrl) {
  let modulePromise = __leptosRerunViewerModules.get(moduleUrl);
  if (!modulePromise) {
    modulePromise = import(/* @vite-ignore */ moduleUrl);
    __leptosRerunViewerModules.set(moduleUrl, modulePromise);
  }

  const module = await modulePromise;
  const WebViewer = module.WebViewer ?? module.default?.WebViewer ?? module.default;
  if (typeof WebViewer !== "function") {
    throw new Error(`Module '${moduleUrl}' does not export a WebViewer class`);
  }
  return WebViewer;
}

function registerEvents(viewer, eventCallback) {
  const unsubscribers = [];
  unsubscribers.push(viewer.on("ready", () => eventCallback({ type: "ready" })));
  unsubscribers.push(
    viewer.on("fullscreen", (fullscreen) =>
      eventCallback({ type: "fullscreen", fullscreen }),
    ),
  );

  for (const eventName of [
    "play",
    "pause",
    "time_update",
    "timeline_change",
    "selection_change",
    "recording_open",
  ]) {
    unsubscribers.push(viewer.on(eventName, (event) => eventCallback(event)));
  }

  return unsubscribers;
}

function normalizeSources(sources) {
  const normalized = new Map();
  for (const source of sources ?? []) {
    if (!source || typeof source.url !== "string" || source.url.length === 0) {
      continue;
    }
    normalized.set(
      source.url,
      Boolean(source.follow_if_http) || normalized.get(source.url) === true,
    );
  }
  return normalized;
}

function normalizePanels(overrides) {
  const normalized = new Map();
  for (const override of overrides ?? []) {
    if (!override || typeof override.panel !== "string") {
      continue;
    }
    normalized.set(override.panel, override.state ?? null);
  }
  return normalized;
}

export async function createViewer(parent, moduleUrl, startupOptions, eventCallback) {
  const WebViewer = await loadWebViewer(moduleUrl);
  const viewer = new WebViewer();
  const bridge = {
    viewer,
    unsubscribers: registerEvents(viewer, eventCallback),
    sources: new Map(),
    panels: new Map(),
  };

  await viewer.start(null, parent, {
    ...startupOptions,
    width: "100%",
    height: "100%",
  });

  return bridge;
}

export function destroyViewer(bridge) {
  for (const unsubscribe of bridge.unsubscribers ?? []) {
    try {
      unsubscribe();
    } catch (_error) {
      // Ignore callback cleanup failures during teardown.
    }
  }
  bridge.unsubscribers = [];
  bridge.viewer.stop();
  bridge.sources = new Map();
  bridge.panels = new Map();
}

export function syncSources(bridge, sources) {
  const nextSources = normalizeSources(sources);

  for (const [url, followIfHttp] of nextSources.entries()) {
    if (!bridge.sources.has(url)) {
      bridge.viewer.open(url, { follow_if_http: followIfHttp });
      continue;
    }

    if (bridge.sources.get(url) !== followIfHttp) {
      bridge.viewer.close(url);
      bridge.viewer.open(url, { follow_if_http: followIfHttp });
    }
  }

  for (const url of bridge.sources.keys()) {
    if (!nextSources.has(url)) {
      bridge.viewer.close(url);
    }
  }

  bridge.sources = nextSources;
}

export function syncPanelOverrides(bridge, overrides, enabled) {
  const nextPanels = normalizePanels(overrides);
  const touchedPanels = new Set([...bridge.panels.keys(), ...nextPanels.keys()]);

  for (const panel of touchedPanels) {
    bridge.viewer.override_panel_state(panel, nextPanels.get(panel) ?? null);
  }

  bridge.viewer.toggle_panel_overrides(Boolean(enabled));
  bridge.panels = nextPanels;
}

export function openSource(bridge, url, followIfHttp) {
  bridge.viewer.open(url, { follow_if_http: Boolean(followIfHttp) });
}

export function closeSource(bridge, url) {
  bridge.viewer.close(url);
}

export function setActiveRecordingId(bridge, recordingId) {
  bridge.viewer.set_active_recording_id(recordingId);
}

export function setPlaying(bridge, recordingId, value) {
  bridge.viewer.set_playing(recordingId, Boolean(value));
}

export function setCurrentTime(bridge, recordingId, timeline, time) {
  bridge.viewer.set_current_time(recordingId, timeline, time);
}

export function setActiveTimeline(bridge, recordingId, timeline) {
  bridge.viewer.set_active_timeline(recordingId, timeline);
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = createViewer)]
    pub(crate) async fn create_viewer(
        parent: web_sys::HtmlElement,
        module_url: &str,
        startup_options: JsValue,
        event_callback: &js_sys::Function,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = destroyViewer)]
    pub(crate) fn destroy_viewer(bridge: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = syncSources)]
    pub(crate) fn sync_sources(bridge: &JsValue, sources: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = syncPanelOverrides)]
    pub(crate) fn sync_panel_overrides(
        bridge: &JsValue,
        overrides: JsValue,
        enabled: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = openSource)]
    pub(crate) fn open_source(
        bridge: &JsValue,
        url: &str,
        follow_if_http: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = closeSource)]
    pub(crate) fn close_source(bridge: &JsValue, url: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = setActiveRecordingId)]
    pub(crate) fn set_active_recording_id(
        bridge: &JsValue,
        recording_id: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = setPlaying)]
    pub(crate) fn set_playing(
        bridge: &JsValue,
        recording_id: &str,
        value: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = setCurrentTime)]
    pub(crate) fn set_current_time(
        bridge: &JsValue,
        recording_id: &str,
        timeline: &str,
        time: f64,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = setActiveTimeline)]
    pub(crate) fn set_active_timeline(
        bridge: &JsValue,
        recording_id: &str,
        timeline: &str,
    ) -> Result<(), JsValue>;
}
