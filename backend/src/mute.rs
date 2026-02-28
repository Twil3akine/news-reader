use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Deserialize)]
pub struct CreateMute {
    pub pattern: String,
}

#[derive(Serialize, FromRow)]
pub struct Mute {
    pub id: i64,
    pub pattern: String,
}

pub async fn list_mute(State(pool): State<SqlitePool>) -> Json<Vec<Mute>> {
    let data = sqlx::   query_as::<_, Mute>("SELECT id, pattern FROM mute_words")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(data)
}

pub async fn create_mute(State(pool): State<SqlitePool>, Json(payload): Json<CreateMute>) {
    sqlx::query("INSERT INTO mute_words (pattern) VALUES (?1)")
        .bind(payload.pattern)
        .execute(&pool)
        .await
        .unwrap();
}

pub async fn delete_mute(State(pool): State<SqlitePool>, Path(id): Path<i64>) {
    sqlx::query("DELETE FROM mute_words WHERE id = ?1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
}
