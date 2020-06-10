# scraper

Rustで作ったnetkeiba.comスクレイパー

## DBのmigration
```bash
cd path/to/keiba
sh bin/start.sh # postgresを起動

cargo install diesel_cli # diesel cliをダウンロード

cd scraper
diesel migration run # migrationの実行
```

## テストの実行

```bash
cargo test -- --show-output
```

## ジョッキーをスクレイプ

```bash
cargo run --bin crawl_jockeys
```

## レースをスクレイプ

```bash
cargo run --bin crawl_races
```
