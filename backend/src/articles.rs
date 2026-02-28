use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Serialize, FromRow)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub published_at: Option<String>,
    pub is_read: i64,
}

pub async fn list_articles(State(pool): State<SqlitePool>) -> Json<Vec<Article>> {
    let mute_words: Vec<String> = sqlx::query_scalar("SELECT pattern FROM mute_words")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let mut articles: Vec<Article> = sqlx::query_as::<_, Article>(
        r#"
            SELECT id, title, url, published_at, is_read
            FROM articles
            ORDER BY is_read ASC, published_at DESC
            "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    articles.retain(|a| !mute_words.iter().any(|m| a.title.contains(m)));

    Json(articles)
}
