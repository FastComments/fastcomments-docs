在 Webhooks 管理介面中，每種事件類型（Create、Update、Delete）都有 `Send Test Payload` 按鈕。Create 和 Update 事件會發送一個測試用的 WebhookComment 物件，而測試 Delete 時只會發送僅含 ID 的測試請求主體。

## 驗證有效負載

在測試您的 webhook 整合時，請確認傳入請求包含以下標頭：

1. **`token`** - 您的 API Secret
2. **`X-FastComments-Timestamp`** - Unix 時間戳（秒）
3. **`X-FastComments-Signature`** - HMAC-SHA256 簽名

使用 HMAC 簽名驗證來確保有效負載的真實性。

## 測試工具

在開發期間，您可以使用 [webhook.site](https://webhook.site) 或 [ngrok](https://ngrok.com) 等工具來檢查傳入的 webhook 有效負載。

## 事件類型

- **Create Event**: 當建立新評論時觸發。預設方法：PUT
- **Update Event**: 當評論被編輯時觸發。預設方法：PUT
- **Delete Event**: 當評論被刪除時觸發。預設方法：DELETE

每個事件在請求主體中包含完整的評論資料（有關有效負載格式，請參見 [資料結構](/guide-webhooks.html#webhooks-structures)）。