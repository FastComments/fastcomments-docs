Agent webhooks 是平台在 agent 執行完成或審核狀態變更時觸發的 HTTP 回呼。您可以使用它們將 agent 活動轉發到您自己的系統——審核儀表板、稽核日誌、Slack 頻道、升級處理工具。

可在 [AI Agents 頁面](https://fastcomments.com/auth/my-account/ai-agents) 的 **Webhooks** 分頁下進行設定。

### 會傳送哪些內容

四種事件類型：

- **`trigger.succeeded`** - agent 執行成功完成。
- **`trigger.failed`** - agent 執行發生錯誤。
- **`approval.requested`** - 某個動作已排入人工審核佇列。
- **`approval.decided`** - 審核已核准、拒絕，或執行失敗。

請參閱 [Webhook 事件](#webhook-events) 了解各事件何時觸發，以及 [Webhook 負載](#webhook-payloads) 了解各自的結構。

### 不會傳送哪些內容

- **個別工具動作的 webhooks。** 呼叫 `pin_comment` 的執行不會為該釘選動作另外觸發 webhook——該動作會包含在執行的 `trigger.succeeded` 負載中。如果您需要個別動作的傳送，請解析觸發負載中的 `actions` 陣列。
- **被捨棄的觸發。** 未派送的觸發（超出預算、範圍錯誤）不會觸發 webhook。捨棄記錄僅在 [Analytics 頁面](#analytics-page) 中可見。
- **重播產生的觸發。** 測試執行不會觸發 webhook。請參閱 [測試執行（重播）](#test-runs-replays)。

### 設定

每個 webhook 您需設定：

- **URL** - 要 POST 的 HTTPS 端點。
- **Domain** - 此 webhook 適用的留言網域（您的租戶可能託管多個網域）。`*` 符合所有網域；`*.example.com` 是 glob 模式；精確網域僅符合該網域。
- **Events** - 要訂閱四種事件類型中的哪些。
- **Agents** - 留空表示「所有 agents」，或填入特定 agent ID 清單進行篩選。
- **Method** - POST 或 PUT（預設為 POST）。
- **No-retry status codes** - 應視為最終失敗、不再重試的 HTTP 回應代碼（例如 410 Gone、422 Unprocessable）。請參閱 [Webhook 重試](#webhook-retries)。

多個 webhooks 可以訂閱同一事件——每個 webhook 獨立排入佇列，並傳送到各自的 URL。

### 依網域比對

給定的事件會傳送到**每個 `domain` 欄位符合該留言網域的 webhook**。比對使用簡單的 glob：

- 精確：`example.com` 僅符合 `example.com`。
- 星號萬用字元：`*` 符合所有網域。
- 子網域 glob：`*.example.com` 符合 `blog.example.com`、`forum.example.com`，但不符合 `example.com` 本身。

負載上的網域取自以下第一個非空值：留言的 `domain`、根據您租戶網域設定解析的 URL，或透過 `urlId` 查詢的 `Page`。

### 依 agent 篩選

**Agents** 欄位讓 webhook 僅訂閱特定的 agents。留空表示「所有 agents」。非空時，webhook 僅會為清單中的 agents 觸發。

當您有一個 webhook 用於審核事件、另一個用於互動事件，且兩者路由到不同的下游系統時，這會很有用。

### 測試傳送

webhook 設定介面有一個 **Test send** 按鈕，會向 URL 傳送一個假的負載，讓您無需等待真實事件即可驗證連線、簽章以及您端點的回應代碼。

### 傳送日誌

每次傳送（以及每次重試）都會記錄在 [Webhook 傳送日誌](#webhook-logs) 頁面，讓您查看線路上實際發生的情況。

### 簽章

每個 webhook 都使用您租戶的 API secret 以 HMAC-SHA256 簽署。請參閱 [Webhook 簽章](#webhook-signing)。

### 資格條件

Agent webhooks 需要租戶具備有效的帳務。它們使用與您現有留言 webhooks 相同的簽章/secret 基礎架構——如果您已經整合過留言 webhooks（請參閱 [Webhooks 指南](/guide-webhooks.html)），agent webhook 的整合方式相同，只是新增了事件類型。