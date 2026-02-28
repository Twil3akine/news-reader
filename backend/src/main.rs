use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

mod articles;
mod db;
mod mute;
mod rss;

#[tokio::main]
async fn main() {
    let pool: SqlitePool = db::init_db().await;

    if let Err(e) = rss::fetch_and_store(&pool).await {
        eprintln!("RSS fetch error: {}", e);
    }

    let app = Router::new()
        .route("/articles", get(articles::list_articles))
        .route("/mute", get(mute::list_mute))
        .route("/mute", post(mute::create_mute))
        .route("/mute/:id", delete(mute::delete_mute))
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
