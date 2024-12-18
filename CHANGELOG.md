# 1.0.1 (December 17, 2024)

### Added
- Update docs, doc links
- Add licenses to crate root
- `wasm_tracing::set_as_global_default_with_config` should not panic by default

# 1.0.0 (December 17, 2024)
## Notes

Thank y'all for bearing with me! I have not been able to find a satisfactory test reference, so I've gone ahead and manually checked different logging configuration options. More importantly, I think this crate's recording implementation is flawed from the ground up. Without overhauling literally every original aspect of `tracing-wasm`, I have opted to find a middle route. Suggestions are highly appreciated!

## Features
- Added `WasmLayerConfig::show_origin`. When disabled, this will remove the line numbers from the console.

## Breaking Changes
### WasmLayerConfig
- `WASMLayerConfigBuilder` has been removed.
  - This builder performed 1:1 logic on `WasmLayerConfig`, and was not useful. The values for `WasmLayerConfig` are now public.
- `WasmLayerConfig` now takes in a `ConsoleConfig` parameter. This replaces `WasmLayerConfig::report_logs_in_console` and `WasmLayerConfig::use_console_color`.
This removes the capability to use a console color while the console is disabled. If there is a use case for this, please let me know.

## Deprecations
- `WASMLayerConfig` has been semantically renamed to `WasmLayerConfig`. A type alias has been marked as deprecated.
- `WASMLayer` has been semantically renamed to `WasmLayer`. A type alias has been marked as deprecated.

## Niceties
- Added more documentation to the crate
- Added a single `wasm_bindgen_test` to visibly check different `WasmLayerConfig` options. This is far from ideal. I plan on expanding functionality of this crate to modify the default global recorder in the future.

**Full Changelog**: https://github.com/dsgallups/wasm-tracing/compare/0.2.0...1.0.0

# 0.2.0 (September 4, 2024)

### Added

- Capture and print span fields (#1)
- Implement support for `log` with `tracing-log` (#3)

# 0.1.0 (September 4, 2024)

### Added

- Cloned initial code from [tracing-wasm](https://github.com/old-storyai/tracing-wasm)
