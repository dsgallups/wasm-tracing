/// Determines how the web console should behave
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConsoleConfig {
    /// Do not record to console
    NoReporting,
    /// Record to console without colorful text
    ReportWithoutConsoleColor,
    /// Record to console with colorful text
    ReportWithConsoleColor,
}

impl ConsoleConfig {
    /// True if the console reporting spans
    pub fn reporting_enabled(&self) -> bool {
        !matches!(self, ConsoleConfig::NoReporting)
    }
}
