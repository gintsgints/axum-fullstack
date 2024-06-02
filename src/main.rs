mod model;
mod router;
mod server;
mod endpoints;

#[tokio::main]
async fn main() {
    server::create_server().await
}
