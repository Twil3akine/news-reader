use axum::{extract::State, Json};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Serialize)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub tags: Option<String>,
    pub published_at: Option<String>,
    pub is_read: i64,
}

pub async fn list_articles(State(pool): State<SqlitePool>) -> Json<Vec<Article>> {
    let rows = sqlx::query(
        r#"
        SELECT id, title, url, tags, published_at, is_read
        FROM articles
        ORDER BY is_read ASC, published_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut articles: Vec<Article> = vec![];

    for row in rows {
        articles.push(Article {
            id: row.get::<i64, _>("id"),
            title: row.get::<String, _>("title"),
            url: row.get::<String, _>("url"),
            tags: row.get::<Option<String>, _>("tags"),
            published_at: row.get::<Option<String>, _>("published_at"),
            is_read: row.get::<i64, _>("is_read"),
        });
    }

    let mute_rows = sqlx::query("SELECT pattern FROM mute_words")
        .fetch_all(&pool)
        .await
        .unwrap();

    let patterns: Vec<String> = mute_rows
        .into_iter()
        .map(|r| r.get::<String, _>("pattern"))
        .collect();

    if patterns.is_empty() {
        return Json(articles);
    }

    let set = RegexSet::new(&patterns).unwrap();

    let filtered = articles
        .into_iter()
        .filter(|a| {
            let target = format!("{} {}", a.title, a.tags.clone().unwrap_or_default());
            !set.is_match(&target)
        })
        .collect();

    Json(filtered)
}

#[derive(Deserialize)]
pub struct ReadUpdate {
    pub id: i64,
}

pub async fn mark_read(State(pool): State<SqlitePool>, Json(payload): Json<ReadUpdate>) {
    sqlx::query("UPDATE articles SET is_read = 1 WHERE id = ?")
        .bind(payload.id)
        .execute(&pool)
        .await
        .unwrap();
}
