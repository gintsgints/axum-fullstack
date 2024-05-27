use std::net::{Ipv4Addr, SocketAddr};
use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tracing::info;
use crate::controller::{get_info_handler, login_handler};
use crate::router;

pub async fn create_server() {
    tracing_subscriber::fmt::init();
    let app = router::build_routes();

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}