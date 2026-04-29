當一個觸發器為某個 agent 觸發但**未**導致 LLM 呼叫時，平台會記錄一個「drop」並標註原因。Drops 會出現在 [Analytics page](#analytics-page) 的「跳過的觸發（本月）」下方。

### 完整的 drop 原因清單

| Reason | What happened |
|---|---|
| `agentDaily` | 已達到該 agent 的每日預算上限。 |
| `agentMonthly` | 已達到該 agent 的每月預算上限。 |
| `tenantDaily` | 已達到租戶的每日預算上限。 |
| `tenantMonthly` | 已達到租戶的每月預算上限。 |
| `qps` | 已達到該 agent 的每分鐘速率限制（滾動 60 秒視窗）。 |
| `concurrency` | 該 agent 的最大併發執行數已被飽和。 |

### 不在此清單中的情形

從未到達 dispatch 路徑的觸發器不會被標記為「drop」——它只是未被 dispatch。這包括：

- Agent 被**停用**。
- 觸發的留言不符合 agent 的 [URL/locale scope](#scope-url-locale)。
- 觸發動作是由同一個 agent 發起（防止迴圈）。
- 租戶有無效的計費資料。
- 該 agent 不在租戶的方案內。

這些都是靜默跳過，而不是 drop。它們不會出現在 Analytics 的 drop 圖表中。

### 在 Analytics 上查看 drops

[Analytics page](#analytics-page) 會顯示：

- **跳過的觸發（本月）** - 依 drop 原因分組的計數。
- **達到或接近上限的代理** - 每個 agent 的明細，顯示哪些 agent 正在接近上限，並列出本期間被 drop 的觸發數量。

### 看到 drops 時該怎麼做

- **`agentDaily` / `agentMonthly`** - 該 agent 的配額太緊。可在編輯表單中提高配額，或縮小 agent 的範圍（URL/locale、更窄的觸發條件）。
- **`tenantDaily` / `tenantMonthly`** - 帳戶層級的配額太緊。可在租戶計費設定中提高，或將費用分配到較少的 agents。
- **`qps`** - 流量觸及每分鐘滾動視窗的限制。通常是討論串快速擴散、觸發速度超過 agent 執行速度的徵兆。agent 的 `maxTriggersPerMinute` 與 `maxConcurrent` 欄位會限制此行為；提高這些值會增加吞吐量，但也會提高突發成本。
- **`concurrency`** - 與 `qps` 相同的根本原因，但表現在進行中（in-flight）的數量上。如果需要更多平行處理，請提高 `maxConcurrent`。

### Drops 與 errors 的差異

drop 表示「觸發器從未執行」。而 **error** 表示「觸發器已執行，但 LLM 呼叫或工具派發失敗」。Errors 會另外在 [Run History](#run-history) 頁面追蹤（狀態為 `Error`）。

### Drops 也會停止重放（replays）

相同的 drop 原因也會停止進行中的 [test runs / replays](#test-runs-replays)。重放將以 Error 狀態停止，並顯示指出哪個預算被擊中的訊息（例如，agent 的每日預算）。

### 防止迴圈的機制故意不記錄為 drop

不會有「此觸發來自另一個 agent 並因防止迴圈而被跳過」的 drop 原因。若記錄此類資訊會讓分析紀錄變得雜亂且無實際訊號——設計上，agent 的擴散（fan-out）不應該導致觸發數暴增。如果你懷疑某個應該被允許的迴圈被不當抑制，請檢查 [Comment Logs](/guide-moderation.html#comment-logs) —— 機器人發表的留言上的 `botId` 正是迴圈檢查的依據。