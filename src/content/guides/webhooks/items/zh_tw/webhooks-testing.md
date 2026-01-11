在 Webhooks 管理介面中，每個事件類型 (Create, Update, Delete) 都有 `Send Test Payload` 按鈕。Create 和 Update 事件會傳送一個範例的 WebhookComment 物件，而測試 Delete 時則會傳送只有 ID 的範例請求主體。

## 驗證有效負載

當測試您的 webhook 整合時，請確認收到的請求包含下列標頭：

1. **`token`** - Your API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (seconds)
3. **`X-FastComments-Signature`** - HMAC-SHA256 signature

使用 HMAC 簽章驗證來確保有效負載的真實性。

## 測試工具

在開發期間，您可以使用像 [webhook.site](https://webhook.site) 或 [ngrok](https://ngrok.com) 這類工具來檢查收到的 webhook 有效負載。

## 事件類型

- **Create Event**: Triggered when a new comment is created. Default method: PUT
- **Update Event**: Triggered when a comment is edited. Default method: PUT
- **Delete Event**: Triggered when a comment is deleted. Default method: DELETE

每個事件在請求主體中包含完整的評論資料（有關有效負載格式，請參閱 [Data Structures](/guides/webhooks/webhooks-structures)）。