/// Determines how the web console should behave
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConsoleConfig {
    reporting: Option<ReportingText>,
}

impl ConsoleConfig {
    pub const fn no_reporting() -> Self {
        Self { reporting: None }
    }
    pub const fn report_without_console_color() -> Self {
        Self {
            reporting: Some(ReportingText::Colorless),
        }
    }
    pub const fn report_with_console_color() -> Self {
        Self {
            reporting: Some(ReportingText::Colorful),
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
