use core::fmt::{self, Write};
use core::sync::atomic::AtomicUsize;
use std::collections::HashMap;

use tracing::Subscriber;
use tracing::{
    dispatcher::SetGlobalDefaultError,
    field::{Field, Visit},
};
#[cfg(feature = "tracing-log")]
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::*;
use tracing_subscriber::registry::*;

use wasm_bindgen::prelude::*;

pub mod config;
pub mod layer;
pub mod recorder;

pub mod prelude {
    pub use super::{
        config::{builder::WASMLayerConfigBuilder, console::ConsoleConfig, WASMLayerConfig},
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_built_config() {
        let builder = WASMLayerConfigBuilder::new();

        let config = builder.build();

        assert_eq!(
            config,
            WASMLayerConfig {
                report_logs_in_timings: true,
                report_logs_in_console: true,
                use_console_color: true,
                max_level: tracing::Level::TRACE,
                show_fields: true,
            }
        )
    }

    #[test]
    fn test_set_report_logs_in_timings() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_report_logs_in_timings(false);

        let config = builder.build();

        assert!(!config.report_logs_in_timings);
    }

    #[test]
    fn test_set_console_config_no_reporting() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_console_config(ConsoleConfig::NoReporting);

        let config = builder.build();

        assert!(!config.report_logs_in_console);
        assert!(!config.use_console_color);
    }

    #[test]
    fn test_set_console_config_without_color() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_console_config(ConsoleConfig::ReportWithoutConsoleColor);

        let config = builder.build();

        assert!(config.report_logs_in_console);
        assert!(!config.use_console_color);
    }

    #[test]
    fn test_set_console_config_with_color() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_console_config(ConsoleConfig::ReportWithConsoleColor);

        let config = builder.build();

        assert!(config.report_logs_in_console);
        assert!(config.use_console_color);
    }

    #[test]
    fn test_default_config_log_level() {
        let builder = WASMLayerConfigBuilder::new();

        let config = builder.build();

        assert_eq!(config.max_level, tracing::Level::TRACE);
    }

    #[test]
    fn test_set_config_log_level_warn() {
        let mut builder = WASMLayerConfigBuilder::new();
        builder.set_max_level(tracing::Level::WARN);

        let config = builder.build();

        assert_eq!(config.max_level, tracing::Level::WARN);
    }
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
