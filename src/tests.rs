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
