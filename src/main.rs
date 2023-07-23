extern crate tracing;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = routes::create_router();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
