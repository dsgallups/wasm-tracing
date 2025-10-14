/// Configurations determining how the web console should behave.
///
/// ### `reporting`
///
/// If this value is `None`, no logging will occur.
/// See [`ReportingText`] for more information.
///
/// ### `use_console_log_levels`
///
/// Enabling this value will emit traces to their corresponding
/// console method.
///
/// This is disabled by default.
///
/// | Level | Console Method |
/// | ----- | -------------- |
/// | `TRACE` | `console.debug` |
/// | `DEBUG` | `console.debug` |
/// | `INFO` | `console.log` |
/// | `WARN` | `console.warn` |
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConsoleConfig {
    pub reporting: Option<ReportingText>,
    /// Enabling this value will emit traces to their corresponding
    /// console method.
    ///
    /// This is disabled by default.
    ///
    /// | Level | Console Method |
    /// | ----- | -------------- |
    /// | `TRACE` | `console.debug` |
    /// | `DEBUG` | `console.debug` |
    /// | `INFO` | `console.log` |
    /// | `WARN` | `console.warn` |
    pub use_console_log_levels: bool,
}

impl ConsoleConfig {
    pub const fn no_reporting() -> Self {
        Self {
            reporting: None,
            use_console_log_levels: false,
        }
    }
    pub const fn report_without_console_color() -> Self {
        Self {
            reporting: Some(ReportingText::Colorless),
            use_console_log_levels: false,
        }
    }
    pub const fn report_with_console_color() -> Self {
        Self {
            reporting: Some(ReportingText::Colorful),
            use_console_log_levels: false,
        }
    }
    /// True if the console reporting spans
    pub const fn reporting_enabled(&self) -> bool {
        self.reporting.is_some()
    }

    pub const fn reporting(&self) -> Option<ReportingText> {
        self.reporting
    }
}

/// Determines how the web console should behave
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReportingText {
    Colorful,
    Colorless,
}
