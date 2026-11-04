每個 agent webhook 都有自己的傳遞日誌。可從 [webhook 清單頁面](https://fastcomments.com/auth/my-account/ai-agents/webhooks) 中每個 webhook 列的 **Logs** 按鈕進入。

### 頁面內容

一個分頁表格，每次傳遞嘗試佔一行：

| 欄位 | 意義 |
|---|---|
| When | 嘗試發生的時間。 |
| Event | 事件類型(`trigger.succeeded`、`approval.requested` 等)。 |
| Status | 傳遞狀態。 |
| StatusCode | 您的端點回傳的 HTTP 狀態碼(若有的話)。 |
| Description | 結果的簡短描述。對於未收到 HTTP 回應的失敗傳遞，底層錯誤訊息會以 `{error: <message>}` 的形式儲存。 |

此頁面僅支援分頁 - 沒有狀態、事件類型或日期範圍的篩選器。

### 您可以從日誌中做什麼

- **判斷某個狀態碼是否應加入 No-retry。** 如果您看到您的端點不斷回傳相同的 `4xx`,這是一個強烈訊號,表示您應將其加入 **No-retry status codes**,讓平台停止重試。

### 失敗資訊

當傳遞失敗且沒有 HTTP 回應時(DNS 失敗、連線被拒、逾時、TLS 錯誤等),原始錯誤訊息會以 `{error: <message>}` 的形式記錄。平台不會將這些歸類為 `TIMEOUT` 或 `DNS_ERROR` 等具名類別 - 請直接閱讀錯誤訊息以了解發生了什麼。

對於 HTTP 回應,StatusCode 欄位會顯示您的端點回傳的狀態碼。常見情況:

- **所有嘗試:`401` 或 `403`** - 您的端點拒絕了簽名。請檢查您是否正確對 `${timestamp}.${body}` 計算 HMAC,並使用正確的密鑰。請參閱 [Webhook Signing](#webhook-signing)。
- **所有嘗試:`422`** - 您的端點認為負載無效。請修正您的端點,或者如果某些事件的拒絕是預期行為,則將 `422` 加入 **No-retry status codes**。

### 每次傳遞的上下文

每條日誌記錄都包含:

- `webhookId` - 產生此次傳遞的 webhook 設定。
- `agentId` - 傳遞所屬的 agent。
- `triggerId` 或 `approvalId` - 底層記錄。
- `domain` - 符合的網域。

您可以使用這些資訊將失敗的傳遞與 [Run History](#run-history) 中相關的執行關聯起來。

### 保留期限

`AgentSyncLog` 記錄在 `createdAt` 上有固定的 1 年 TTL,無論結果如何 - 成功與失敗的傳遞保留時間相同。超過保留期限後,日誌記錄即被刪除。

如果您需要長期稽核,可持續的做法是:讓**端點本身**保存其收到的傳遞。這樣可將您的稽核日誌與平台的保留政策解耦。

### 測試傳送

webhook 設定表單的 **Test send** 按鈕會將一次模擬傳遞寫入同一個日誌表格,讓您在依賴真實事件之前驗證端對端連線。測試傳遞在日誌中會有明確標記,因此不會污染生產環境的失敗統計數據。

### 另請參閱

- [Webhooks Overview](#webhooks-overview)。
- [Webhook Retries](#webhook-retries) - 驅動這些日誌的重試語義。
- [Webhook Signing](#webhook-signing) - 如何在您的端點進行驗證。