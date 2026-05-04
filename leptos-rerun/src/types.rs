use std::collections::BTreeMap;

use leptos::prelude::{Get, Memo, ReadSignal, RwSignal, Signal, Storage};
use serde::{Deserialize, Serialize};

pub const DEFAULT_RERUN_CDN_VERSION: &str = "0.31.3";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetOrigin {
    JsDelivr { version: String },
    CustomModule { url: String },
}

impl AssetOrigin {
    pub fn jsdelivr(version: impl Into<String>) -> Self {
        Self::JsDelivr {
            version: version.into(),
        }
    }

    pub fn custom_module(url: impl Into<String>) -> Self {
        Self::CustomModule { url: url.into() }
    }

    pub fn module_url(&self) -> String {
        match self {
            Self::JsDelivr { version } => {
                format!("https://cdn.jsdelivr.net/npm/@rerun-io/web-viewer@{version}/+esm")
            }
            Self::CustomModule { url } => url.clone(),
        }
    }
}

impl Default for AssetOrigin {
    fn default() -> Self {
        Self::jsdelivr(DEFAULT_RERUN_CDN_VERSION)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderBackend {
    Webgpu,
    Webgl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoDecoder {
    Auto,
    PreferSoftware,
    PreferHardware,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Rrds(Vec<String>);

impl Rrds {
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> From<Vec<T>> for Rrds
where
    T: Into<String>,
{
    fn from(value: Vec<T>) -> Self {
        Self(value.into_iter().map(Into::into).collect())
    }
}

impl<T, const N: usize> From<[T; N]> for Rrds
where
    T: Into<String>,
{
    fn from(value: [T; N]) -> Self {
        Self(value.into_iter().map(Into::into).collect())
    }
}

impl From<String> for Rrds {
    fn from(value: String) -> Self {
        Self(vec![value])
    }
}

impl From<&str> for Rrds {
    fn from(value: &str) -> Self {
        Self(vec![value.to_string()])
    }
}

#[derive(Clone)]
pub struct RrdProp(Signal<Rrds>);

impl RrdProp {
    pub fn get(&self) -> Rrds {
        self.0.get()
    }
}

impl Default for RrdProp {
    fn default() -> Self {
        Self(Signal::from(Rrds::default()))
    }
}

impl From<Rrds> for RrdProp {
    fn from(value: Rrds) -> Self {
        Self(Signal::from(value))
    }
}

impl<T> From<Vec<T>> for RrdProp
where
    T: Into<String>,
{
    fn from(value: Vec<T>) -> Self {
        Self::from(Rrds::from(value))
    }
}

impl<T, const N: usize> From<[T; N]> for RrdProp
where
    T: Into<String>,
{
    fn from(value: [T; N]) -> Self {
        Self::from(Rrds::from(value))
    }
}

impl From<String> for RrdProp {
    fn from(value: String) -> Self {
        Self::from(Rrds::from(value))
    }
}

impl From<&str> for RrdProp {
    fn from(value: &str) -> Self {
        Self::from(Rrds::from(value))
    }
}

impl<T> From<Signal<T>> for RrdProp
where
    T: Clone + Into<Rrds> + Send + Sync + 'static,
{
    fn from(value: Signal<T>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

impl<T, S> From<ReadSignal<T, S>> for RrdProp
where
    ReadSignal<T, S>: Get<Value = T> + Clone + 'static,
    T: Clone + Into<Rrds> + Send + Sync + 'static,
{
    fn from(value: ReadSignal<T, S>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

impl<T, S> From<RwSignal<T, S>> for RrdProp
where
    RwSignal<T, S>: Get<Value = T> + Clone + 'static,
    T: Clone + Into<Rrds> + Send + Sync + 'static,
{
    fn from(value: RwSignal<T, S>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

impl<T, S> From<Memo<T, S>> for RrdProp
where
    Memo<T, S>: Get<Value = T> + Clone + 'static,
    S: Storage<T>,
    T: Clone + Into<Rrds> + Send + Sync + 'static,
{
    fn from(value: Memo<T, S>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Panel {
    Top,
    Blueprint,
    Selection,
    Time,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelState {
    Hidden,
    Collapsed,
    Expanded,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PanelStateOverrides(BTreeMap<Panel, PanelState>);

impl PanelStateOverrides {
    pub fn iter(&self) -> impl Iterator<Item = (Panel, PanelState)> + '_ {
        self.0.iter().map(|(&panel, &state)| (panel, state))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<(Panel, PanelState)>> for PanelStateOverrides {
    fn from(value: Vec<(Panel, PanelState)>) -> Self {
        Self(value.into_iter().collect())
    }
}

impl<const N: usize> From<[(Panel, PanelState); N]> for PanelStateOverrides {
    fn from(value: [(Panel, PanelState); N]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl From<BTreeMap<Panel, PanelState>> for PanelStateOverrides {
    fn from(value: BTreeMap<Panel, PanelState>) -> Self {
        Self(value)
    }
}

#[derive(Clone)]
pub struct PanelStateOverridesProp(Signal<PanelStateOverrides>);

impl PanelStateOverridesProp {
    pub fn get(&self) -> PanelStateOverrides {
        self.0.get()
    }
}

impl Default for PanelStateOverridesProp {
    fn default() -> Self {
        Self(Signal::from(PanelStateOverrides::default()))
    }
}

impl From<PanelStateOverrides> for PanelStateOverridesProp {
    fn from(value: PanelStateOverrides) -> Self {
        Self(Signal::from(value))
    }
}

impl From<Vec<(Panel, PanelState)>> for PanelStateOverridesProp {
    fn from(value: Vec<(Panel, PanelState)>) -> Self {
        Self::from(PanelStateOverrides::from(value))
    }
}

impl<const N: usize> From<[(Panel, PanelState); N]> for PanelStateOverridesProp {
    fn from(value: [(Panel, PanelState); N]) -> Self {
        Self::from(PanelStateOverrides::from(value))
    }
}

impl From<BTreeMap<Panel, PanelState>> for PanelStateOverridesProp {
    fn from(value: BTreeMap<Panel, PanelState>) -> Self {
        Self::from(PanelStateOverrides::from(value))
    }
}

impl<T> From<Signal<T>> for PanelStateOverridesProp
where
    T: Clone + Into<PanelStateOverrides> + Send + Sync + 'static,
{
    fn from(value: Signal<T>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

impl<T, S> From<ReadSignal<T, S>> for PanelStateOverridesProp
where
    ReadSignal<T, S>: Get<Value = T> + Clone + 'static,
    T: Clone + Into<PanelStateOverrides> + Send + Sync + 'static,
{
    fn from(value: ReadSignal<T, S>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

impl<T, S> From<RwSignal<T, S>> for PanelStateOverridesProp
where
    RwSignal<T, S>: Get<Value = T> + Clone + 'static,
    T: Clone + Into<PanelStateOverrides> + Send + Sync + 'static,
{
    fn from(value: RwSignal<T, S>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

impl<T, S> From<Memo<T, S>> for PanelStateOverridesProp
where
    Memo<T, S>: Get<Value = T> + Clone + 'static,
    S: Storage<T>,
    T: Clone + Into<PanelStateOverrides> + Send + Sync + 'static,
{
    fn from(value: Memo<T, S>) -> Self {
        Self(Signal::derive(move || value.get().into()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerEventBase {
    pub application_id: String,
    pub recording_id: String,
    #[serde(default)]
    pub partition_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SelectionItem {
    Entity {
        entity_path: String,
        #[serde(default)]
        instance_id: Option<u64>,
        #[serde(default)]
        view_name: Option<String>,
        #[serde(default)]
        position: Option<[f64; 3]>,
    },
    View {
        view_id: String,
        view_name: String,
    },
    Container {
        container_id: String,
        container_name: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RerunViewerEvent {
    Ready,
    Fullscreen {
        fullscreen: bool,
    },
    Play {
        #[serde(flatten)]
        base: ViewerEventBase,
    },
    Pause {
        #[serde(flatten)]
        base: ViewerEventBase,
    },
    TimeUpdate {
        #[serde(flatten)]
        base: ViewerEventBase,
        time: f64,
    },
    TimelineChange {
        #[serde(flatten)]
        base: ViewerEventBase,
        timeline: String,
        time: f64,
    },
    SelectionChange {
        #[serde(flatten)]
        base: ViewerEventBase,
        items: Vec<SelectionItem>,
    },
    RecordingOpen {
        #[serde(flatten)]
        base: ViewerEventBase,
        source: String,
        #[serde(default)]
        version: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_origin_default_uses_pinned_jsdelivr_version() {
        assert_eq!(
            AssetOrigin::default().module_url(),
            format!(
                "https://cdn.jsdelivr.net/npm/@rerun-io/web-viewer@{DEFAULT_RERUN_CDN_VERSION}/+esm"
            )
        );
    }

    #[test]
    fn custom_module_origin_preserves_url() {
        let url = "https://example.com/rerun/viewer.js";
        assert_eq!(AssetOrigin::custom_module(url).module_url(), url);
    }

    #[test]
    fn selection_change_event_deserializes_from_json() {
        let event = serde_json::from_str::<RerunViewerEvent>(
            r#"{
                "type": "selection_change",
                "application_id": "demo-app",
                "recording_id": "rec-1",
                "partition_id": null,
                "items": [
                    {
                        "type": "entity",
                        "entity_path": "/world/camera",
                        "instance_id": 42,
                        "view_name": "3D View",
                        "position": [1.0, 2.0, 3.0]
                    }
                ]
            }"#,
        )
        .expect("event should deserialize");

        assert_eq!(
            event,
            RerunViewerEvent::SelectionChange {
                base: ViewerEventBase {
                    application_id: "demo-app".to_string(),
                    recording_id: "rec-1".to_string(),
                    partition_id: None,
                },
                items: vec![SelectionItem::Entity {
                    entity_path: "/world/camera".to_string(),
                    instance_id: Some(42),
                    view_name: Some("3D View".to_string()),
                    position: Some([1.0, 2.0, 3.0]),
                }],
            }
        );
    }

    #[test]
    fn rrds_accept_single_or_multiple_urls() {
        assert_eq!(
            Rrds::from("https://example.com/one.rrd").as_slice(),
            ["https://example.com/one.rrd".to_string()]
        );
        assert_eq!(
            Rrds::from(["https://example.com/one.rrd", "https://example.com/two.rrd",]).as_slice(),
            [
                "https://example.com/one.rrd".to_string(),
                "https://example.com/two.rrd".to_string(),
            ]
        );
    }

    #[test]
    fn panel_state_overrides_keep_one_value_per_panel() {
        let overrides = PanelStateOverrides::from([
            (Panel::Top, PanelState::Hidden),
            (Panel::Time, PanelState::Collapsed),
            (Panel::Top, PanelState::Expanded),
        ]);

        assert_eq!(
            overrides.iter().collect::<Vec<_>>(),
            vec![
                (Panel::Top, PanelState::Expanded),
                (Panel::Time, PanelState::Collapsed),
            ]
        );
    }
}
