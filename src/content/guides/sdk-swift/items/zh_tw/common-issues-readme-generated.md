### 401 未授權錯誤

如果在使用已驗證的 API 時收到 401 錯誤：

1. **檢查您的 API 金鑰**：確保您使用的是 FastComments 儀表板中正確的 API 金鑰
2. **驗證租戶 ID**：確保租戶 ID 與您的帳戶相符
3. **API 金鑰格式**：API 金鑰應在共享設定中以 `x-api-key` 標頭設定：

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **使用錯誤的 API**：確保在已驗證的呼叫中使用 `DefaultAPI`（而非 `PublicAPI`）

### SSO 令牌問題

如果 SSO 令牌無法運作：

1. **在生產環境使用安全模式**：在生產環境中，始終使用帶有您的 API 金鑰的 `FastCommentsSSO.createSecure()`
2. **僅限伺服器端**：在您的伺服器上產生安全的 SSO 令牌，切勿將 API 金鑰暴露給客戶端
3. **檢查使用者資料**：確保提供所有必要欄位（id、email、username）
4. **令牌過期**：安全的 SSO 令牌包含時間戳記，可能會過期。請根據需要產生新的令牌。

### SSL/TLS 錯誤

如果您遇到 SSL/TLS 錯誤：

1. 確保應用程式的 Info.plist 允許連接至 fastcomments.com 的 HTTPS 連線
2. 檢查您是否未使用可能阻擋連線的 App Transport Security 例外設定