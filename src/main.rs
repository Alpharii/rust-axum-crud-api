mod db;
mod models;
mod handlers;
mod routes;

use axum::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = db::init_pool().await;
    let app = routes::create_routes(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
