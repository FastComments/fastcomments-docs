Run History 是每個代理的觸發器執行記錄。可從代理列表頁面透過 **Runs** 按鈕進入，或直接前往 `/auth/my-account/ai-agents/{agentId}/runs`。

### 頁面內容

一個分頁表格，每次執行佔一行：

| 欄位 | 意義 |
|---|---|
| Date | 觸發器觸發的時間（或延遲觸發器執行的時間）。 |
| Status | **Started**、**Success** 或 **Error**。若執行處於 dry-run 模式，會同時顯示 **Dry Run** 標籤。 |
| Cost | 每次執行的成本，以您的租戶貨幣計算。進行中（Started）的執行此欄為空。 |
| Actions | 該次執行中的工具呼叫次數。 |
| Details | 一個 **View** 按鈕，可開啟 [Run Detail View](#run-detail-view)。 |

### 狀態意義

- **Started** - 執行正在進行中，或在完成前已中止。若執行異常長時間停留在「Started」，通常代表 LLM 呼叫逾時。
- **Error** - 執行已完成但在某處失敗 - LLM 呼叫回傳錯誤、工具分派失敗等。詳細檢視中包含具體錯誤。
- **Success** - 執行完成且無錯誤。代理可能採取了零個、一個或多個動作。

### 空狀態

當代理沒有任何執行記錄時，頁面會顯示：「No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments.」

最後那句是有意為之的 - [測試執行流程](#test-runs-replays) 是在新代理上填充 Run History 的建議方式。

### 執行歷史頁面上沒有的內容

- **從未分派的即時觸發器** - 因預算、範圍或速率限制而被捨棄的觸發器不會出現在此頁面。這些會顯示在 [Analytics 頁面](#analytics-page) 的「Triggers skipped」下方。
- **審批** - 此執行中所採取動作的待審批項目位於 [審批收件匣](#approval-workflow)。該動作會在執行詳細檢視中顯示為 **Pending approval**。

### 保留期限

個別執行記錄會保留 90 天，之後該執行將從歷史中移除。成本與觸發器計數仍會持續彙總至長期分析摘要中，因此 [Analytics 頁面](#analytics-page) 在該期限之後仍會顯示歷史總計。

### 重播

由重播產生的執行預設不會顯示在即時執行檢視中。您可以在 [Test Runs (Replays)](#test-runs-replays) 頁面查看這些記錄。

### 跨代理篩選

執行表格是按代理區分的。沒有跨代理的執行檢視 - [Analytics 頁面](#analytics-page) 才是跨代理的摘要。如果您需要檢視多個代理的執行情況，可以將 [Webhooks](#webhooks-overview) 的 `trigger.succeeded` 與 `trigger.failed` 事件轉發到您自己的系統。