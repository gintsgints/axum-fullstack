use crate::endpoints::UsersRouter;
use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::{HeaderMap, Request};
use axum::response::Response;
use axum::routing::get_service;
use axum::Router;
use std::time::Duration;
use tower_http::services::ServeDir;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info, info_span, Span};

pub async fn build_routes() -> Router {
    Router::new()
        .nest("/api", UsersRouter::new_router())
        .fallback_service(get_service(ServeDir::new("./web/dist")))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
    
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }).on_request(|_request: &Request<_>, _span: &Span| {
                info!("request");
            })
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                // ...
            })
            .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                // ...
            })
            .on_eos(
                |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                    // ...
                },
            )
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    // ...
                },
            )
        )
}
