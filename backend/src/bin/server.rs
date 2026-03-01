use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, sqlite::SqlitePoolOptions};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[derive(Deserialize)]
struct ArticleQuery {
    page: Option<u32>,
}

#[derive(Serialize, FromRow, Clone)]
struct Article {
    id: i64,
    title: String,
    url: String,
    source: String,
    published_at: String,
    is_read: bool,
}

#[derive(Deserialize)]
struct MuteWordPayload {
    word: String,
}

#[derive(Serialize, FromRow)]
struct MuteWord {
    id: i64,
    word: String,
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://news.db")
        .await
        .unwrap();

    let state = AppState { pool };

    let app = Router::new()
        .route("/api/articles", get(get_articles))
        .route("/api/articles/{id}/read", post(mark_as_read))
        .route("/api/mutewords", get(get_mutewords).post(add_muteword))
        .route("/api/mutewords/{id}", delete(delete_muteword))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleQuery>,
) -> Json<Vec<Article>> {
    let mut query = String::from(
        "SELECT id, title, url, source, published_at, is_read
         FROM articles
         WHERE NOT EXISTS (
             SELECT 1 FROM mute_words WHERE LOWER(articles.title) LIKE '%' || LOWER(mute_words.word) || '%'
         )
         ORDER BY published_at DESC"
    );

    let limit = 50;
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * limit;
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let articles = sqlx::query_as::<_, Article>(&query)
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

    Json(articles)
}

async fn mark_as_read(State(state): State<AppState>, Path(id): Path<i64>) -> Json<()> {
    let _ = sqlx::query("UPDATE articles SET is_read = 1 WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await;
    Json(())
}

async fn get_mutewords(State(state): State<AppState>) -> Json<Vec<MuteWord>> {
    let words =
        sqlx::query_as::<_, MuteWord>("SELECT id, word FROM mute_words ORDER BY created_at DESC")
            .fetch_all(&state.pool)
            .await
            .unwrap_or_default();
    Json(words)
}

async fn add_muteword(
    State(state): State<AppState>,
    Json(payload): Json<MuteWordPayload>,
) -> Json<()> {
    if !payload.word.trim().is_empty() {
        let _ =
            sqlx::query("INSERT INTO mute_words (word) VALUES (?) ON CONFLICT(word) DO NOTHING")
                .bind(payload.word.trim())
                .execute(&state.pool)
                .await;
    }
    Json(())
}

async fn delete_muteword(State(state): State<AppState>, Path(id): Path<i64>) -> Json<()> {
    let _ = sqlx::query("DELETE FROM mute_words WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await;
    Json(())
}
