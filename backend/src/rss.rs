use rss::Channel;
use sqlx::SqlitePool;
use std::io::Cursor;

pub async fn fetch_and_store(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()?;

    let url = "https://zenn.dev/feed";

    let response = client.get(url).send().await?;
    let body = response.bytes().await?;

    let channel = Channel::read_from(Cursor::new(body))?;

    for item in channel.items() {
        let title = item.title().unwrap_or("").to_string();
        let link = item.link().unwrap_or("").to_string();
        let pub_date = item.pub_date().unwrap_or("").to_string();

        sqlx::query(
            r#"
            INSERT OR IGNORE INTO articles (title, url, published_at)
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(title)
        .bind(link)
        .bind(pub_date)
        .execute(pool)
        .await?;
    }

    Ok(())
}
