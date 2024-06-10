use crate::endpoints::UsersRouter;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;

pub async fn build_routes() -> Router {
    Router::new()
        .nest("/api", UsersRouter::new_router())
        .fallback_service(get_service(ServeDir::new("./web/dist")))
}
