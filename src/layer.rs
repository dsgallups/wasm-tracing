use std::sync::atomic::AtomicUsize;

use crate::prelude::*;

/// Implements [tracing_subscriber::layer::Layer] which uses [wasm_bindgen] for marking and measuring with `window.performance`
pub struct WASMLayer {
    last_event_id: AtomicUsize,
    config: WASMLayerConfig,
}

impl WASMLayer {
    pub fn new(config: WASMLayerConfig) -> Self {
        WASMLayer {
            last_event_id: AtomicUsize::new(0),
            config,
        }
    }
}

impl core::default::Default for WASMLayer {
    fn default() -> Self {
        WASMLayer::new(WASMLayerConfig::default())
    }
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for WASMLayer {
    fn enabled(&self, metadata: &tracing::Metadata<'_>, _: Context<'_, S>) -> bool {
        let level = metadata.level();
        level <= &self.config.max_level
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::Id,
        ctx: Context<'_, S>,
    ) {
        let mut new_debug_record = StringRecorder::new(self.config.show_fields);
        attrs.record(&mut new_debug_record);

        if let Some(span_ref) = ctx.span(id) {
            span_ref
                .extensions_mut()
                .insert::<StringRecorder>(new_debug_record);
        }
    }

    /// doc: Notifies this layer that a span with the given Id recorded the given values.
    fn on_record(&self, id: &tracing::Id, values: &tracing::span::Record<'_>, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.span(id) {
            if let Some(debug_record) = span_ref.extensions_mut().get_mut::<StringRecorder>() {
                values.record(debug_record);
            }
        }
    }

    // /// doc: Notifies this layer that a span with the ID span recorded that it follows from the span with the ID follows.
    // fn on_follows_from(&self, _span: &tracing::Id, _follows: &tracing::Id, ctx: Context<'_, S>) {}
    /// doc: Notifies this layer that an event has occurred.
    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        if self.config.report_logs_in_timings || self.config.report_logs_in_console {
            let mut recorder = StringRecorder::new(self.config.show_fields);
            event.record(&mut recorder);
            #[cfg(feature = "tracing-log")]
            let normalized_meta = event.normalized_metadata();
            #[cfg(feature = "tracing-log")]
            let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());
            #[cfg(not(feature = "tracing-log"))]
            let meta = event.metadata();
            let level = meta.level();
            if self.config.report_logs_in_console {
                let origin = meta
                    .file()
                    .and_then(|file| meta.line().map(|ln| format!("{}:{}", file, ln)))
                    .unwrap_or_default();

                let fields = ctx
                    .lookup_current()
                    .and_then(|span| {
                        span.extensions()
                            .get::<StringRecorder>()
                            .map(|span_recorder| {
                                span_recorder
                                    .fields
                                    .iter()
                                    .map(|(key, value)| format!("\n\t{key}: {value}"))
                                    .collect::<Vec<_>>()
                                    .join("")
                            })
                    })
                    .unwrap_or_default();

                if self.config.use_console_color {
                    log4(
                        format!(
                            "%c{}%c {}{}%c{}{}",
                            level,
                            origin,
                            thread_display_suffix(),
                            recorder,
                            fields
                        ),
                        match *level {
                            tracing::Level::TRACE => "color: dodgerblue; background: #444",
                            tracing::Level::DEBUG => "color: lawngreen; background: #444",
                            tracing::Level::INFO => "color: whitesmoke; background: #444",
                            tracing::Level::WARN => "color: orange; background: #444",
                            tracing::Level::ERROR => "color: red; background: #444",
                        },
                        "color: gray; font-style: italic",
                        "color: inherit",
                    );
                } else {
                    log1(format!(
                        "{} {}{} {}{}",
                        level,
                        origin,
                        thread_display_suffix(),
                        recorder,
                        fields
                    ));
                }
            }
            if self.config.report_logs_in_timings {
                let mark_name = format!(
                    "c{:x}",
                    self.last_event_id
                        .fetch_add(1, core::sync::atomic::Ordering::Relaxed)
                );
                // mark and measure so you can see a little blip in the profile
                mark(&mark_name);
                let _ = measure(
                    format!(
                        "{} {}{} {}",
                        level,
                        meta.module_path().unwrap_or("..."),
                        thread_display_suffix(),
                        recorder,
                    ),
                    mark_name,
                );
            }
        }
    }
    /// doc: Notifies this layer that a span with the given ID was entered.
    fn on_enter(&self, id: &tracing::Id, _ctx: Context<'_, S>) {
        mark(&mark_name(id));
    }
    /// doc: Notifies this layer that the span with the given ID was exited.
    fn on_exit(&self, id: &tracing::Id, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.span(id) {
            let meta = span_ref.metadata();
            if let Some(debug_record) = span_ref.extensions().get::<StringRecorder>() {
                let _ = measure(
                    format!(
                        "\"{}\"{} {} {}",
                        meta.name(),
                        thread_display_suffix(),
                        meta.module_path().unwrap_or("..."),
                        debug_record,
                    ),
                    mark_name(id),
                );
            } else {
                let _ = measure(
                    format!(
                        "\"{}\"{} {}",
                        meta.name(),
                        thread_display_suffix(),
                        meta.module_path().unwrap_or("..."),
                    ),
                    mark_name(id),
                );
            }
        }
    }
    // /// doc: Notifies this layer that the span with the given ID has been closed.
    // /// We can dispose of any data for the span we might have here...
    // fn on_close(&self, _id: tracing::Id, ctx: Context<'_, S>) {}
    // /// doc: Notifies this layer that a span ID has been cloned, and that the subscriber returned a different ID.
    // /// I'm not sure if I need to do something here...
    // fn on_id_change(&self, _old: &tracing::Id, _new: &tracing::Id, ctx: Context<'_, S>) {}
}
