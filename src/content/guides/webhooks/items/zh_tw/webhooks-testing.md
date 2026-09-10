新的和編輯 webhook 頁面都有一個 `Send Test Payload` 按鈕，會將請求發送到表單中目前的 URL，無論是否已儲存。Create 和 Update 事件會傳送一個虛擬的 WebhookComment 物件，而測試 Delete 時會傳送僅包含 ID 的虛擬請求主體。

## 驗證有效負載

測試 webhook 整合時，請確認傳入的請求包含以下標頭：

1. **`X-FastComments-Timestamp`** - Unix 時間戳記（秒）
2. **`X-FastComments-Signature`** - HMAC-SHA256 簽名

在引入簽名機制之前建立的 webhook 也會收到包含您 API 密鑰的 **`token`** 標頭。新 webhook 則不會。

使用 HMAC 簽名驗證以確保 payload 為真實的。

## 測試工具

您可以使用像是 [webhook.site](https://webhook.site) 或 [ngrok](https://ngrok.com) 之類的工具，在開發過程中檢查傳入的 webhook payload。

## 事件類型

- **Create Event**：當新評論被建立時觸發。
- **Update Event**：當評論被編輯時觸發。
- **Delete Event**：當評論被刪除時觸發。

每個 webhook 皆綁定單一事件與單一 HTTP 方法（POST、PUT 或 DELETE）。每個事件在請求主體中包含完整的評論資料（請參閱 [Data Structures](/guide-webhooks.html#webhooks-structures) 了解 payload 格式）。

---