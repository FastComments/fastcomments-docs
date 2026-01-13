### 401 Unauthorized エラー

認証済み API を使用していて 401 エラーが発生している場合:

1. **APIキーを確認する**: FastComments のダッシュボードにある正しい API キーを使用していることを確認してください
2. **テナント ID を確認する**: テナント ID がアカウントと一致していることを確認してください
3. **API キーの形式**: API キーは Configuration に渡す必要があります:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO トークンの問題

SSO トークンが機能していない場合:

1. **本番環境ではセキュアモードを使用する**: 本番環境では必ず `FastCommentsSSO::new_secure()` を API キーとともに使用してください
2. **サーバー側のみで生成する**: SSO トークンはサーバー上で生成し、API キーをクライアントに公開しないでください
3. **ユーザーデータを確認する**: 必須フィールド (id, email, username) がすべて提供されていることを確認してください

### 非同期ランタイムのエラー

SDK は非同期処理に tokio を使用しています。次の点を確認してください:

1. 依存関係に tokio を追加する:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. tokio ランタイムを使用する:
```rust
#[tokio::main]
async fn main() {
    // ここに非同期コードを記述してください
}
```