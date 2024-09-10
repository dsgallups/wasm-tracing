use tracing::dispatcher::SetGlobalDefaultError;
use tracing_subscriber::layer::*;
use tracing_subscriber::registry::*;

use wasm_bindgen::prelude::*;

pub mod config;
pub mod layer;
pub(crate) mod recorder;

pub mod prelude {
    pub use super::{
        config::{console::ConsoleConfig, WASMLayerConfig},
        layer::WASMLayer,
    };
}
use prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn mark(a: &str);
    #[wasm_bindgen(catch, js_namespace = performance)]
    fn measure(name: String, startMark: String) -> Result<(), JsValue>;
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log1(message: String);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log2(message1: &str, message2: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log3(message1: &str, message2: &str, message3: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log4(message1: String, message2: &str, message3: &str, message4: &str);
}

#[cfg(not(feature = "mark-with-rayon-thread-index"))]
#[inline]
fn thread_display_suffix() -> &'static str {
    ""
}
#[cfg(feature = "mark-with-rayon-thread-index")]
fn thread_display_suffix() -> String {
    let mut message = " #".to_string();
    match rayon::current_thread_index() {
        Some(idx) => message.push_str(&format!("{}", idx)),
        None => message.push_str("main"),
    }
    message
}

#[cfg(not(feature = "mark-with-rayon-thread-index"))]
fn mark_name(id: &tracing::Id) -> String {
    format!("t{:x}", id.into_u64())
}
#[cfg(feature = "mark-with-rayon-thread-index")]
fn mark_name(id: &tracing::Id) -> String {
    format!(
        "t{:x}-{}",
        id.into_u64(),
        rayon::current_thread_index().unwrap_or(999)
    )
}

/// Set the global default with [tracing::subscriber::set_global_default]
pub fn set_as_global_default() {
    tracing::subscriber::set_global_default(
        Registry::default().with(WASMLayer::new(WASMLayerConfig::default())),
    )
    .expect("default global");
}

/// Set the global default with [tracing::subscriber::set_global_default]
pub fn try_set_as_global_default() -> Result<(), SetGlobalDefaultError> {
    tracing::subscriber::set_global_default(
        Registry::default().with(WASMLayer::new(WASMLayerConfig::default())),
    )
}

/// Set the global default with [tracing::subscriber::set_global_default]
pub fn set_as_global_default_with_config(config: WASMLayerConfig) {
    tracing::subscriber::set_global_default(Registry::default().with(WASMLayer::new(config)))
        .expect("default global");
}
