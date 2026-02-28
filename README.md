# News Reader

## 概要
自分専用RSSビューア

対象: Zenn RSS
保存: SQLite
ミュート: titleベース

## 起動方法

cargo run

http://localhost:3000/articles

## API

GET /articles
GET /mute
POST /mute
DELETE /mute/:id

## 仕様

- 300件取得
- INSERT OR IGNORE
- 未読優先
- 新着順
