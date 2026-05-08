# rust-drills (with code)

1日1時間 × 2ヶ月想定のRustドリル。**Day01〜Day60の“動くサンプルコード”付き**。

## コンテナ運用

```bash
docker build -t rust-drills \
  --build-arg UID=$(id -u) \
  --build-arg GID=$(id -g) \
  .
chmod +x rust.sh

# Day01
./rust.sh cargo run -p day01_cli_sum -- 10 -2 5
./rust.sh cargo test -p day01_cli_sum
```

## 注意

- Day36以降は tokio/axum/serde 等の依存が増えます。
- ルートで `cargo test --workspace` を回すと全日分のビルドになり重いので、基本は `-p dayXX_...` で回すのがおすすめ。
- 依存クレートは初回にネットワークから取得します。
