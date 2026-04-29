一個 **審核** 是一個排入佇列的工具呼叫，平台在執行前需要由人員批准或拒絕。

### Configuring approvals

在代理編輯表單上，**Approvals** 區段列出代理被允許調用的每個工具（允許清單），並讓您勾選那些必須由人工審查的工具。未勾選的工具會立即執行。勾選的工具會被排入佇列。

被拒絕的工具會被*直接拒絕*，不會排入佇列——審核僅適用於其他情況下被允許的工具。

### What happens when a gated action fires

1. 代理挑選一個工具呼叫（例如 `ban_user`）以及參數、理由和信心水準。
2. 平台不會執行該呼叫，而是將一個處於 `PENDING` 狀態的審核排入佇列，包含工具名稱、參數、理由、信心水準，以及描述觸發來源的上下文快照。
3. 通知會發送給審核者（參見 [Approval Notifications](#approval-notifications)）。
4. 代理的執行完成並被記錄——在 [Run Detail View](#run-detail-view) 中該動作會顯示為 **等待審核**。

### Reviewing approvals

位於 [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) 的審核收件匣會列出待處理、已批准、已拒絕和執行失敗的審核。對於每一項：

- **Tool name and arguments** - 代理欲執行的內容（精確）。
- **Agent reasoning** - 代理提供的理由。
- **Confidence** - 代理自評的信心水準，介於 0.0 到 1.0。
- **Context snapshot** - 該動作所針對的評論、頁面與使用者。

兩個按鈕：**Approve** 與 **Reject**。Approve 會實際執行該工具；Reject 則會捨棄該請求。

### Approval status states

一個審核會經過這些狀態：

| State | Meaning |
|---|---|
| `PENDING` | 等待人工決定。 |
| `APPROVED` | 已被人工批准且動作已執行。 |
| `REJECTED` | 已被人工拒絕。該動作未執行。 |
| `EXECUTION_FAILED` | 已被人工批准但執行失敗（例如目標評論已被刪除）。 |
| `EXECUTING` | 短暫狀態：人工點選 Approve 後動作正在執行。用於序列化同時的批准點擊，確保有實際副作用的工具不會被執行兩次。 |

當兩名審核者同時點選 Approve 時，`EXECUTING` 狀態很重要——一人會成功，另一人會看到該審核已經改變狀態。

### What gets cleaned up

待處理的審核會維持待處理狀態直到有人決定。它們會在建立後 **90 天** 自動過期。任何其他狀態的審核也會為了儲存維護而在 90 天後清除。

審核的「決定者（decided by）」/「決定時間（decided at）」/「執行時間（executed at）」/「執行結果（execution result）」欄位會隨著審核狀態變動而被填入——所有這些都可在收件匣的詳細檢視中看到。

### Webhook integration

當審核狀態變動時，會觸發兩個 webhook 事件：

- **`approval.requested`** - 在插入為 `PENDING` 時觸發。
- **`approval.decided`** - 在轉為 `APPROVED`、`REJECTED` 或 `EXECUTION_FAILED` 時觸發。

兩者的簽名方式與其他 webhook 相同。請參閱 [Webhook Events](#webhook-events) 與 [Webhook Payloads](#webhook-payloads)。

### Hardening: known-tool gate

審核機制會拒絕將任何非已識別代理工具名稱排入佇列。這是一道深度防禦檢查：即便未來某個程式路徑將由 LLM 產生的工具名稱傳入審核流程，那個字串也永遠無法成為排隊的審核項目。已知工具名稱的集合與 [Tools Reference](#tools-overview) 中列出的集合相同。

### Common gating patterns

- **Brand-new moderation agent** - 對 `ban_user`、`mark_comment_spam`、`mark_comment_approved`、`send_email` 設置閘控。觀察收件匣幾週，之後可從低錯誤率的工具移除閘控。
- **Long-term moderation agent** - 保持 `ban_user` 以及任何不可逆的操作（`deleteAllUsersComments`, `banIP`）永久受閘控。
- **EU region** - 不論您勾選與否，依據第 17 條，`ban_user` 在 Article 17 下會被強制啟用。請參閱 [EU DSA Article 17 Compliance](#eu-dsa-compliance)。

### What approvals do **not** do

- 它們不會延遲代理的其他工具呼叫。代理的一次執行可能呼叫多個工具，只有受閘控的那些會排入佇列——其餘的會照常執行。
- 若人工拒絕，它們不會回滾代理的執行。未受閘控的執行部分已經完成。