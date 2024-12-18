#[doc(hidden)]
mod console;
pub use console::*;

#[deprecated(since = "1.0.0", note = "Rename WASMLayerConfig to WasmLayerConfig.")]
pub type WASMLayerConfig = WasmLayerConfig;

#[doc = r#"
Configuration parameters for the [WasmLayer](crate::prelude::WasmLayer).
"#]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct WasmLayerConfig {
    pub report_logs_in_timings: bool,
    pub console: ConsoleConfig,
    /// Maximum log level
    pub max_level: tracing::Level,
    /// Show/hide fields of types
    pub show_fields: bool,
    /// Show origin (line number, source)
    pub show_origin: bool,
}

impl Default for WasmLayerConfig {
    fn default() -> Self {
        WasmLayerConfig {
            report_logs_in_timings: true,
            console: ConsoleConfig::ReportWithConsoleColor,
            max_level: tracing::Level::TRACE,
            show_fields: true,
            show_origin: true,
        }
    }
}

impl WasmLayerConfig {
    pub fn new() -> WasmLayerConfig {
        WasmLayerConfig::default()
    }

    /// Set whether events should appear in performance Timings
    pub fn set_report_logs_in_timings(&mut self, report_logs_in_timings: bool) -> &mut Self {
        self.report_logs_in_timings = report_logs_in_timings;
        self
    }

    /// Set the maximal level on which events should be displayed
    pub fn set_max_level(&mut self, max_level: tracing::Level) -> &mut Self {
        self.max_level = max_level;
        self
    }

    /// Set if and how events should be displayed in the browser console
    pub fn set_console_config(&mut self, console_config: ConsoleConfig) -> &mut Self {
        self.console = console_config;
        self
    }

    pub fn set_show_origin(&mut self, show_origin: bool) -> &mut Self {
        self.show_origin = show_origin;
        self
    }

    /// Set if events will show additional fields, usually the file or line.
    pub fn set_show_fields(&mut self, show_fields: bool) -> &mut Self {
        self.show_fields = show_fields;
        self
    }

    pub fn console_enabled(&self) -> bool {
        self.console.reporting_enabled()
    }
}

#[test]
fn test_default_built_config() {
    let config = WasmLayerConfig::new();

    assert_eq!(
        config,
        WasmLayerConfig {
            report_logs_in_timings: true,
            console: ConsoleConfig::ReportWithConsoleColor,
            max_level: tracing::Level::TRACE,
            show_fields: true,
            show_origin: true
        }
    )
}

#[test]
fn test_set_report_logs_in_timings() {
    let mut config = WasmLayerConfig::new();
    config.set_report_logs_in_timings(false);

    assert!(!config.report_logs_in_timings);
}

#[test]
fn test_set_console_config_no_reporting() {
    let mut config = WasmLayerConfig::new();
    config.set_console_config(ConsoleConfig::NoReporting);

    assert!(!config.console.reporting_enabled());
}

#[test]
fn test_set_console_config_without_color() {
    let mut config = WasmLayerConfig::new();
    config.set_console_config(ConsoleConfig::ReportWithoutConsoleColor);

    assert_eq!(config.console, ConsoleConfig::ReportWithoutConsoleColor);
}

#[test]
fn test_set_console_config_with_color() {
    let mut config = WasmLayerConfig::new();
    config.set_console_config(ConsoleConfig::ReportWithConsoleColor);

    assert_eq!(config.console, ConsoleConfig::ReportWithConsoleColor);
}

#[test]
fn test_set_config_log_level_warn() {
    let mut config = WasmLayerConfig::new();
    config.set_max_level(tracing::Level::WARN);

    assert_eq!(config.max_level, tracing::Level::WARN);
}
