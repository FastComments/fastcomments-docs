### 401 未授權錯誤

如果您在使用需要驗證的 API 時遇到 401 錯誤：

1. **檢查您的 API 金鑰**：確保您使用的是來自 FastComments 儀表板的正確 API 金鑰
2. **驗證租戶 ID**：確保租戶 ID 與您的帳戶相符
3. **API 金鑰格式**：API 金鑰應透過 Configuration 傳遞：

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO 權杖問題

如果 SSO 權杖無法正常運作：

1. **在正式環境使用安全模式**：在正式環境中，請務必搭配您的 API 金鑰使用 `FastCommentsSSO::new_secure()`
2. **僅在伺服器端**：在您的伺服器上產生 SSO 權杖，切勿將您的 API 金鑰暴露給用戶端
3. **檢查使用者資料**：確保提供所有必要欄位（id、email、username）

### 非同步執行時錯誤

此 SDK 使用 tokio 進行非同步操作。請務必：

1. 將 tokio 加入您的相依性：
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. 使用 tokio 執行時：
```rust
#[tokio::main]
async fn main() {
    // 您的非同步程式碼
}
```