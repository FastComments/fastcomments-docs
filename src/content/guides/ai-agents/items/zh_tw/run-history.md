Run History 是每個代理的已執行觸發器日誌。可從代理列表頁面透過 **Runs** 按鈕存取，或直接前往 `/auth/my-account/ai-agents/{agentId}/runs`。

### What's on the page

A paginated table with one row per run:

| Column | Meaning |
|---|---|
| Date | 當觸發器觸發時（或延遲觸發器執行時）的時間。 |
| Status | **Started**、**Success** 或 **Error**。若該執行為 dry-run 模式，則會一併顯示 **Dry Run** 徽章。 |
| Cost | 以您租戶貨幣計算的每次執行成本。正在進行中的（Started）執行顯示為空白。 |
| Actions | 該次執行中工具呼叫的次數。 |
| Details | 一個會開啟 [Run Detail View](#run-detail-view) 的 **View** 按鈕。 |

### Status meanings

- **Started** - 該次執行正在進行中，或在完成前中止。長時間停留在「Started」通常代表 LLM 呼叫逾時。
- **Error** - 該次執行已完成但發生錯誤——例如 LLM 回傳錯誤、工具調度失敗等。詳細檢視中會包含具體錯誤內容。
- **Success** - 該次執行已成功完成，無錯誤。代理可能未採取任何動作、或採取了一次或多次動作。

### Empty state

當代理沒有任何執行紀錄時，頁面會顯示：「No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments.」

最後這段是刻意的——建議使用 [測試執行流程](#test-runs-replays) 來在全新代理上填充執行記錄。

### What's not on the run history page

- **Live triggers that never dispatched** - 因預算、範圍或速率限制而被取消的觸發器不會出現在此頁面。這些會顯示在 [Analytics page](#analytics-page) 的「Triggers skipped」中。
- **Approvals** - 此次執行中所採取動作的待審核項目位於 [approvals inbox](#approval-workflow)。該動作會在執行詳細檢視中顯示為 **Pending approval**。

### Retention

個別執行紀錄會保留 90 天，逾期後該執行會從記錄中移除。成本與觸發器計數會繼續彙整至長期分析摘要，因此 [Analytics page](#analytics-page) 仍會顯示超出該時段的歷史總計。

### Replays

由回放產生的執行預設不包含在即時執行檢視中。您可以在 [測試執行（回放）](#test-runs-replays) 頁面查看那些執行。

### Filtering across agents

執行表格是以代理為單位。沒有跨代理的執行檢視——[Analytics page](#analytics-page) 才是跨代理的彙總。如果您需要檢查多個代理的執行，應將 [Webhooks](#webhooks-overview) 的 `trigger.succeeded` 與 `trigger.failed` 事件轉發到您自己的系統。