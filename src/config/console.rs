#[doc = r#"
Sets [WasmLayerConfig::report_logs_in_console](super::WasmLayerConfig::report_logs_in_console) and
[WasmLayerConfig::use_console_color](super::WasmLayerConfig::use_console_color) together.
"#]
pub enum ConsoleConfig {
    /// Do not record to console
    NoReporting,
    /// Record to console without colorful text
    ReportWithoutConsoleColor,
    /// Record to console with colorful text
    ReportWithConsoleColor,
}
