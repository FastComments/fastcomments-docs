Agent webhooks 在失敗時會重試。交付對代理來說是**一次發送即忘（fire-and-forget）**——傳送失敗不會阻塞代理執行或回滾任何操作——重試由隊列加上排程（cron）非同步驅動。

### Queue model

每個事件會**針對每一個匹配的 webhook 排入隊列一次**。所以如果對某個 agent + domain 有三個 webhook 訂閱了 `trigger.succeeded`，平台會排入三筆傳送；每一筆獨立傳送並獨立重試。一個 webhook 的失敗永遠不會影響其他 webhook。

### What's retried

當發生下列情況時會重試傳送：

- HTTP 請求**未完成**（DNS 解析失敗、連線被拒、逾時）。
- HTTP 回應碼為任何非 `2xx` 的狀態，且該狀態不在設定的 **No-retry status codes** 清單中。

以下情況**不會重試**：

- 回應碼為 `2xx`（成功）。
- 回應碼在設定的 **No-retry status codes** 清單中。預設此清單為空——任何非 `2xx` 都會被重試。

### Configuring no-retry codes

webhook 設定表單有個 **No-retry status codes** 欄位（多值）。常見條目：

- `410` - Gone。您的端點已永久移除或資源不存在。重試只會浪費雙方頻寬。
- `422` - Unprocessable Entity。您的端點理解了負載但判定其無效。用相同的負載重試會得到相同的回應。
- `400` - Bad Request，意義相同。

在此加入一個狀態碼表示：當端點回傳該碼時，將該次傳送標記為失敗且為終止（failed-terminal），停止重試。

### Retry schedule

背景工作器每隔幾秒會執行一次，處理所有其下一次嘗試時間已過的傳送項目。

每次失敗後，下一次嘗試時間會以**線性退避（linear backoff）**向後推移：等待時間隨著嘗試次數成長，計算方式為 `60 seconds * attempt count`（所以第 1 次嘗試前等待 1 分鐘，第 2 次等待 2 分鐘，依此類推）。

在 99 次失敗後（或在本地開發為 3 次），該傳送會被放棄並從隊列中刪除。傳送日誌條目仍會被保留，並在 [Webhook 傳送日誌](#webhook-logs) 頁面中可見直到它們到期。

### Idempotence on your side

因為我們會重試，您的端點**必須是冪等的**。相同的 `triggerId`（或 `approvalId`）可能會被重複送達。您的端點應該：

- 使用唯一鍵（觸發事件使用 `triggerId`，核准事件使用 `approvalId`）作為去重令牌（dedup token）。
- 優雅地接受重複傳送（第二次回傳 200）。

非冪等的端點最終會重複處理部分傳送，特別是在短暫中斷期間，原始請求其實已成功但因逾時被重試（例如 30 秒後重試）時。

### Ordering

傳送**不保證嚴格順序**。來自同一次執行的 `trigger.succeeded` 與下游的 `approval.requested` 可能會因為其中一方重試而以任一順序到達。您的端點不應假定因果順序。

如果您需要順序性，請使用時間戳——信封上的 `occurredAt`，以及資料區塊中的觸發/核准 `createdAt`——在您這端重建順序。

### Cleanup

傳送一旦成功或達到嘗試上限即會從隊列中移除。平台不會在隊列中保留終止失敗的傳送；每次嘗試的持久記錄會保存在 [Webhook 傳送日誌](#webhook-logs) 頁面。

### Where to look when retries fail

[Webhook 傳送日誌](#webhook-logs) 頁面是查看 webhook 為何失敗的地方。常見原因：

- **DNS 解析失敗** - URL 錯誤或網域不存在。
- **TLS 錯誤** - 您端點的憑證無效或已過期。
- **連線被拒 / 逾時** - 您的端點當機。
- **5xx 回應** - 您的端點有啟動但發生錯誤。回應主體（截斷）會被記錄。
- **4xx 回應** - 您的端點拒絕了負載。如果這是預期行為，請將該狀態碼新增至 **No-retry status codes**。

### Pause an unhealthy webhook

如果某個 webhook 持續失敗，最乾淨的修正方式是刪除它（或暫時清空其事件訂閱清單）。平台不會自動停用失敗的 webhook——它們會持續重試直到該傳送被放棄。