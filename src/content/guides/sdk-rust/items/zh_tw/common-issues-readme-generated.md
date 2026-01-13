### 401 Unauthorized Errors

如果在使用需要認證的 API 時收到 401 錯誤：

1. **檢查您的 API 金鑰**：確保您正在使用來自 FastComments 儀表板的正確 API 金鑰
2. **驗證租戶 ID**：確保租戶 ID 與您的帳戶相符
3. **API 金鑰格式**：API 金鑰應該在 Configuration 中傳遞：

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO Token Issues

如果 SSO 令牌無法運作：

1. **於生產環境使用安全模式**：在生產環境中始終使用 `FastCommentsSSO::new_secure()` 並搭配您的 API 金鑰
2. **僅限伺服端**：在您的伺服器上產生 SSO 令牌，切勿將 API 金鑰洩露給客戶端
3. **檢查使用者資料**：確保所有必要欄位（id, email, username）都已提供

### Async Runtime Errors

SDK 使用 tokio 執行非同步操作。請確保：

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Your async code here
}
```