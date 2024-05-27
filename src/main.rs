mod controller;
mod model;
mod router;
mod server;

#[tokio::main]
async fn main() {
    server::create_server().await
}
