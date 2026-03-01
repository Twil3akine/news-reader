use feed_rs::parser;
use reqwest::Client;
use sqlx::sqlite::SqlitePoolOptions;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePoolOptions::new().connect("sqlite://news.db").await?;

    // User-Agentを偽装
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;

    // RSSフィードのリスト（HatenaはIT総合、ZDNetは正しいURLに修正）
    let feeds = vec![
        ("Zenn Feed", "https://zenn.dev/feed"),
        ("Qiita Feed", "https://qiita.com/popular-items/feed"),
        ("Hatena IT Feed", "https://b.hatena.ne.jp/hotentry/it.rss"),
        ("ZDNet Feed", "https://japan.zdnet.com/rss/news/index.rdf"),
    ];

    for (feed_name, feed_url) in feeds {
        println!("Fetching {}...", feed_name);

        let response = match client.get(feed_url).send().await {
            Ok(res) => {
                if !res.status().is_success() {
                    eprintln!("  -> HTTP Error: {} (Status: {})", feed_name, res.status());
                    continue;
                }
                res
            }
            Err(e) => {
                eprintln!("  -> Network Error {}: {}", feed_name, e);
                continue;
            }
        };

        let content = match response.bytes().await {
            Ok(b) => b,
            Err(e) => {
                eprintln!("  -> Read Error {}: {}", feed_name, e);
                continue;
            }
        };

        let feed = match parser::parse(&content[..]) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("  -> Parse Error {}: {}", feed_name, e);
                continue;
            }
        };

        let mut new_count = 0;
        for entry in feed.entries {
            let title = entry.title.map(|t| t.content).unwrap_or_default();
            let link = entry
                .links
                .first()
                .map(|l| l.href.clone())
                .unwrap_or_default();
            let pub_date = entry
                .published
                .or(entry.updated)
                .map(|d| d.to_rfc3339())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

            if title.is_empty() || link.is_empty() {
                continue;
            }

            // 記事のURLを解析し、ドメインから「ソース名」を動的に判定する
            let source_name: String = if let Ok(parsed_url) = Url::parse(&link) {
                match parsed_url.host_str().unwrap_or("Others") {
                    "zenn.dev" => "Zenn".to_string(),
                    "qiita.com" => "Qiita".to_string(),
                    host if host.contains("hatenablog.com") || host.contains("hatena.blog") => {
                        "Hatena Blog".to_string()
                    }
                    "japan.zdnet.com" => "ZDNet".to_string(),
                    _ => "Others".to_string(), // Zenn, Qiita, Hatena Blog, ZDNet以外はすべてOthers
                }
            } else {
                "Others".to_string()
            };

            let result = sqlx::query(
                "INSERT INTO articles (title, url, source, published_at)
                 VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING",
            )
            .bind(&title)
            .bind(&link)
            .bind(&source_name)
            .bind(&pub_date)
            .execute(&pool)
            .await;

            if let Ok(res) = result {
                if res.rows_affected() > 0 {
                    new_count += 1;
                }
            }
        }
        println!("  -> Fetched and saved {} new articles.", new_count);
    }

    println!("Fetch completed.");
    Ok(())
}
