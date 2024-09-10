use super::{console::ConsoleConfig, WASMLayerConfig};

pub struct WASMLayerConfigBuilder {
    /// Log events will be marked and measured so they appear in performance Timings
    report_logs_in_timings: bool,
    /// Log events will be logged to the browser console
    report_logs_in_console: bool,
    /// Only relevant if report_logs_in_console is true, this will use color style strings in the console.
    use_console_color: bool,
    /// Log events will be reported from this level -- Default is ALL (TRACE)
    max_level: tracing::Level,
    /// Log events will show additional fields, usually the file or line.
    show_fields: bool,
}

impl WASMLayerConfigBuilder {
    pub fn new() -> WASMLayerConfigBuilder {
        WASMLayerConfigBuilder::default()
    }

    /// Set whether events should appear in performance Timings
    pub fn set_report_logs_in_timings(
        &mut self,
        report_logs_in_timings: bool,
    ) -> &mut WASMLayerConfigBuilder {
        self.report_logs_in_timings = report_logs_in_timings;
        self
    }

    /// Set the maximal level on which events should be displayed
    pub fn set_max_level(&mut self, max_level: tracing::Level) -> &mut WASMLayerConfigBuilder {
        self.max_level = max_level;
        self
    }

    /// Set if and how events should be displayed in the browser console
    pub fn set_console_config(
        &mut self,
        console_config: ConsoleConfig,
    ) -> &mut WASMLayerConfigBuilder {
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
    pub fn set_show_fields(&mut self, show_fields: bool) -> &mut WASMLayerConfigBuilder {
        self.show_fields = show_fields;
        self
    }

    /// Build the WASMLayerConfig
    pub fn build(&self) -> WASMLayerConfig {
        WASMLayerConfig {
            report_logs_in_timings: self.report_logs_in_timings,
            report_logs_in_console: self.report_logs_in_console,
            use_console_color: self.use_console_color,
            max_level: self.max_level,
            show_fields: self.show_fields,
        }
    }
}

impl Default for WASMLayerConfigBuilder {
    fn default() -> WASMLayerConfigBuilder {
        WASMLayerConfigBuilder {
            report_logs_in_timings: true,
            report_logs_in_console: true,
            use_console_color: true,
            max_level: tracing::Level::TRACE,
            show_fields: true,
        }
    }
}
