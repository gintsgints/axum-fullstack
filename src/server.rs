use crate::router;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tracing::info;

pub async fn create_server() {
    tracing_subscriber::fmt::init();
    let app = router::build_routes().await;

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
