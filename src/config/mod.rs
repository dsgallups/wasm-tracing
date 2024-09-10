pub mod builder;

pub mod console;

#[derive(Debug, PartialEq)]
pub struct WASMLayerConfig {
    report_logs_in_timings: bool,
    report_logs_in_console: bool,
    use_console_color: bool,
    max_level: tracing::Level,
    show_fields: bool,
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
