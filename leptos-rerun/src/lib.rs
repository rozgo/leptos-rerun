#![cfg_attr(feature = "ssr", deny(rust_2018_idioms))]

mod bridge;
pub mod components;
pub mod prelude;
pub mod types;

pub use crate::types::DEFAULT_RERUN_CDN_VERSION;
