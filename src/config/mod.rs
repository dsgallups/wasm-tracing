use crate::ConsoleConfig;

pub mod console;

#[derive(Debug, PartialEq)]
pub struct WASMLayerConfig {
    pub report_logs_in_timings: bool,
    pub report_logs_in_console: bool,
    pub use_console_color: bool,
    pub max_level: tracing::Level,
    pub show_fields: bool,
}

impl Default for WASMLayerConfig {
    fn default() -> Self {
        WASMLayerConfig {
            report_logs_in_timings: true,
            report_logs_in_console: true,
            use_console_color: true,
            max_level: tracing::Level::TRACE,
            show_fields: true,
        }
    }
}

impl WASMLayerConfig {
    pub fn new() -> WASMLayerConfig {
        WASMLayerConfig::default()
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
        match console_config {
            ConsoleConfig::NoReporting => {
                self.report_logs_in_console = false;
                self.use_console_color = false;
            }
            ConsoleConfig::ReportWithoutConsoleColor => {
                self.report_logs_in_console = true;
                self.use_console_color = false;
            }
            ConsoleConfig::ReportWithConsoleColor => {
                self.report_logs_in_console = true;
                self.use_console_color = true;
            }
        }

        self
    }

    /// Set if events will show additional fields, usually the file or line.
    pub fn set_show_fields(&mut self, show_fields: bool) -> &mut Self {
        self.show_fields = show_fields;
        self
    }
}

#[test]
fn test_default_built_config() {
    let config = WASMLayerConfig::new();

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
    let mut config = WASMLayerConfig::new();
    config.set_report_logs_in_timings(false);

    assert!(!config.report_logs_in_timings);
}

#[test]
fn test_set_console_config_no_reporting() {
    let mut config = WASMLayerConfig::new();
    config.set_console_config(ConsoleConfig::NoReporting);

    assert!(!config.report_logs_in_console);
    assert!(!config.use_console_color);
}

#[test]
fn test_set_console_config_without_color() {
    let mut config = WASMLayerConfig::new();
    config.set_console_config(ConsoleConfig::ReportWithoutConsoleColor);

    assert!(config.report_logs_in_console);
    assert!(!config.use_console_color);
}

#[test]
fn test_set_console_config_with_color() {
    let mut config = WASMLayerConfig::new();
    config.set_console_config(ConsoleConfig::ReportWithConsoleColor);

    assert!(config.report_logs_in_console);
    assert!(config.use_console_color);
}

#[test]
fn test_set_config_log_level_warn() {
    let mut config = WASMLayerConfig::new();
    config.set_max_level(tracing::Level::WARN);

    assert_eq!(config.max_level, tracing::Level::WARN);
}
