每個 agent webhook 都有自己的傳送記錄。可從 [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) 上每個 webhook 列的 **Logs** 按鈕進入。

### What's on the page

帶分頁的表格，每次傳送嘗試一列：

| Column | Meaning |
|---|---|
| When | 嘗試發生的時間。 |
| Event | 事件類型 (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | 傳送狀態。 |
| StatusCode | 您的端點回傳的 HTTP 狀態碼（若可得）。 |
| Description | 結果的簡短描述。對於未收到 HTTP 回應而失敗的傳送，底層錯誤訊息會被儲存為 {error: <message>}。 |

該頁面只支援分頁 — 沒有狀態、事件類型或日期範圍篩選功能。

### What you can do from logs

- **決定某個狀態碼是否應列入 不重試的狀態碼（No-retry status codes）。** 如果您看到您的端點不斷回傳相同的 `4xx`，那就是一個強烈的跡象，表示您會想把它加入 **不重試的狀態碼**，以便平台停止重試。

### Failure information

當傳送失敗而未收到 HTTP 回應（DNS 失敗、連線被拒、逾時、TLS 錯誤等）時，原始錯誤訊息會記錄為 {error: <message>}。平台不會把這些分類為像 `TIMEOUT` 或 `DNS_ERROR` 這類命名的桶；請直接閱讀錯誤訊息以了解發生了什麼。

對於 HTTP 回應，StatusCode 欄位顯示您的端點回傳的代碼。常見情況：

- **所有嘗試：`401` 或 `403`** - 您的端點正在拒絕簽章。檢查您是否對 `${timestamp}.${body}` 計算 HMAC，並使用正確的密鑰。請參閱 [Webhook Signing](#webhook-signing)。
- **所有嘗試：`422`** - 您的端點認為負載無效。要么修正您的端點，要么如果某些事件預期會被拒絕，將 `422` 加入 **不重試的狀態碼**。

### Per-delivery context

每個日誌條目都包含：

- `webhookId` - 哪個 webhook 設定產生此傳送。
- `agentId` - 此傳送所屬的 agent。
- `triggerId` or `approvalId` - 底層紀錄。
- `domain` - 匹配到的網域。

您可以使用這些來將失敗的傳送與 [Run History](#run-history) 中該次運行關聯起來。

### Retention

`AgentSyncLog` 條目在 `createdAt` 上有固定 1 年的 TTL，與結果無關 — 成功與失敗的傳送都會保留相同的時間。超過保留期後，該日誌條目會被移除。

如果您需要長期稽核，較可持續的做法是：讓 **端點本身** 保存它收到的傳送。這會將您的稽核日誌與平台的保留政策解耦。

### Test send

webhook 設定表單的 **Test send** 按鈕會在相同的日誌表中寫入一個假的傳送，以便您在依賴真實事件前驗證端到端的連通性。測試傳送在日誌中有明確標示，因此不會污染生產的失敗統計。

### See also

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) 了解驅動這些日誌的重試語意。
- [Webhook Signing](#webhook-signing) 了解如何在您的端點驗證。