# keiba
競馬予想アプリ(作成中)

## 目的
- Rust学びたい
- rust-cpython使ってみたい
- 競馬データってオープンソースなのに意外と分析されてない
- 機械学習学びたい
  - EDA職人の技を真似して盗む
  - Linear model, LightGBM, XGBoost, Catboost, NNなど一通りのモデル試してモデルの気持ちを理解する
- MLOps学びたい
  - mlflow
  - kedro
  - nyaggle
  - gokart
  - hydra

## 技術選定
- scraper -> Rust
- machine learning -> python
- API -> rust-cpython
- frontend -> React / wasm(Rust学びたいだけ) / Elm

## ロードマップ
- [x] netkeiba.com scraper
  - [x] 騎手
  - [x] レース結果
  - [x] レース
- [doing] Study Kedro 
- [ ] EDA
- [ ] Create CV
- [ ] Create simple model
- [ ] More EDA
- [ ] Improve model
- [ ] Create API
- [ ] Web frontend

- [ ] additional netkeiba.com scraper
  - [ ] 馬
  - [ ] 種牡馬
  - [ ] 馬主
  - [ ] 調教師
  - [ ] 生産者

- [ ] Improve model

## Setup

```bash
sh bin/init.sh
```
