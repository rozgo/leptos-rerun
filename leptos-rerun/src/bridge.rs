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

function registerEvents(bridge, eventCallback) {
  const { viewer } = bridge;
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
    unsubscribers.push(viewer.on(eventName, (event) => {
      handlePlaybackEvent(bridge, event);
      eventCallback(event);
    }));
  }

  return unsubscribers;
}

function normalizePlaybackOptions(options) {
  return {
    autoplay: Boolean(options?.autoplay),
    loopPlayback: Boolean(options?.loop_playback),
    timeline:
      typeof options?.timeline === "string" && options.timeline.length > 0
        ? options.timeline
        : null,
  };
}

function playbackRecordingId(event) {
  return typeof event?.recording_id === "string" && event.recording_id.length > 0
    ? event.recording_id
    : null;
}

function playbackTimeline(bridge, recordingId, event) {
  if (bridge.playbackOptions.timeline) {
    return bridge.playbackOptions.timeline;
  }
  if (typeof event?.timeline === "string" && event.timeline.length > 0) {
    return event.timeline;
  }
  try {
    return bridge.viewer.get_active_timeline(recordingId);
  } catch (_error) {
    return null;
  }
}

function configurePlayback(bridge, recordingId, event) {
  const { autoplay, timeline } = bridge.playbackOptions;
  if (!autoplay && !timeline) {
    return;
  }

  window.setTimeout(() => {
    const activeTimeline = playbackTimeline(bridge, recordingId, event);
    try {
      if (timeline && activeTimeline) {
        bridge.viewer.set_active_timeline(recordingId, activeTimeline);
      }
      if (autoplay) {
        bridge.viewer.set_playing(recordingId, true);
      }
    } catch (_error) {
      // The recording may have been closed before the deferred setup ran.
    }
  }, 0);
}

function maybeLoopPlayback(bridge, recordingId, event) {
  if (!bridge.playbackOptions.loopPlayback) {
    return;
  }

  const timeline = playbackTimeline(bridge, recordingId, event);
  if (!timeline) {
    return;
  }

  let range;
  let time;
  try {
    range = bridge.viewer.get_time_range(recordingId, timeline);
    time = typeof event?.time === "number"
      ? event.time
      : bridge.viewer.get_current_time(recordingId, timeline);
  } catch (_error) {
    return;
  }

  if (!range || typeof range.min !== "number" || typeof range.max !== "number") {
    return;
  }
  const span = range.max - range.min;
  if (!Number.isFinite(span) || span <= 0 || !Number.isFinite(time)) {
    return;
  }

  const tolerance = Math.max(Math.abs(span) * 0.001, 0.01);
  if (time < range.max - tolerance) {
    return;
  }

  const loopKey = `${recordingId}:${timeline}`;
  const now = Date.now();
  if ((bridge.loopResets.get(loopKey) ?? 0) + 250 > now) {
    return;
  }
  bridge.loopResets.set(loopKey, now);

  try {
    bridge.viewer.set_current_time(recordingId, timeline, range.min);
    bridge.viewer.set_playing(recordingId, true);
  } catch (_error) {
    // Ignore transient viewer state while sources are changing.
  }
}

function handlePlaybackEvent(bridge, event) {
  const recordingId = playbackRecordingId(event);
  if (!recordingId) {
    return;
  }
  bridge.recordings.add(recordingId);

  if (event?.type === "recording_open") {
    configurePlayback(bridge, recordingId, event);
    return;
  }

  if (event?.type === "timeline_change" && bridge.playbackOptions.autoplay) {
    configurePlayback(bridge, recordingId, event);
  }

  if (event?.type === "time_update" || event?.type === "pause") {
    maybeLoopPlayback(bridge, recordingId, event);
  }
}

function normalizeSources(sources) {
  const normalized = new Map();
  for (const source of sources ?? []) {
    if (!source || typeof source.url !== "string" || source.url.length === 0) {
      continue;
    }
    const browserUrl = toBrowserUrl(source.url);
    normalized.set(
      browserUrl,
      Boolean(source.follow_if_http) || normalized.get(browserUrl) === true,
    );
  }
  return normalized;
}

function toBrowserUrl(url) {
  if (typeof url !== "string" || url.length === 0) {
    return url;
  }

  try {
    return new URL(url, window.location.href).toString();
  } catch (_error) {
    return url;
  }
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
    unsubscribers: [],
    sources: new Map(),
    panels: new Map(),
    playbackOptions: normalizePlaybackOptions(null),
    loopResets: new Map(),
    recordings: new Set(),
  };
  bridge.unsubscribers = registerEvents(bridge, eventCallback);

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

export function syncPlaybackOptions(bridge, options) {
  bridge.playbackOptions = normalizePlaybackOptions(options);
  for (const recordingId of bridge.recordings) {
    configurePlayback(bridge, recordingId, null);
  }
}

export function openSource(bridge, url, followIfHttp) {
  bridge.viewer.open(toBrowserUrl(url), { follow_if_http: Boolean(followIfHttp) });
}

export function closeSource(bridge, url) {
  bridge.viewer.close(toBrowserUrl(url));
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

    #[wasm_bindgen(catch, js_name = syncPlaybackOptions)]
    pub(crate) fn sync_playback_options(bridge: &JsValue, options: JsValue) -> Result<(), JsValue>;

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
