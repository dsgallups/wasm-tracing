pub mod builder;

pub mod console;

#[derive(Debug, PartialEq)]
pub struct WASMLayerConfig {
    pub(crate) report_logs_in_timings: bool,
    pub(crate) report_logs_in_console: bool,
    pub(crate) use_console_color: bool,
    pub(crate) max_level: tracing::Level,
    pub(crate) show_fields: bool,
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
