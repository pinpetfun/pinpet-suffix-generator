use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing::Level;

pub fn logging_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new()
            .level(Level::INFO)
            .include_headers(true))
        .on_response(DefaultOnResponse::new()
            .level(Level::INFO)
            .include_headers(true))
}