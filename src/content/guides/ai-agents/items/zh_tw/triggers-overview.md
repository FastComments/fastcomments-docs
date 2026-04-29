A **觸發器** 是一個喚醒代理的事件。每個代理可以定義一個或多個觸發器。

### The full list

| Trigger | When it fires |
|---|---|
| [Comment Added](#trigger-comment-add) | 發表新的評論。 |
| [Comment Edited](#trigger-comment-edit) | 評論被編輯。先前的文字會包含在代理的上下文中。 |
| [Comment Deleted](#trigger-comment-delete) | 評論被刪除。 |
| [Comment Pinned](#trigger-comment-pin) | 評論被釘選（任何人，包括版主或其他代理）。 |
| [Comment Unpinned](#trigger-comment-unpin) | 評論被取消釘選。 |
| [Comment Locked](#trigger-comment-lock) | 評論被鎖定（不允許進一步回覆）。 |
| [Comment Unlocked](#trigger-comment-unlock) | 評論被解除鎖定。 |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | 評論的淨票數達到設定的門檻。 |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | 評論的檢舉數恰好達到設定的門檻。 |
| [User Posts First Comment](#trigger-new-user-first-comment) | 使用者在此站點發表第一則評論。 |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | 評論被垃圾檢測引擎自動標記為垃圾。 |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | 版主將評論標記為已審核。 |
| [Moderator Approves Comment](#trigger-moderator-approved) | 版主核准評論。 |
| [Moderator Marks Spam](#trigger-moderator-spammed) | 版主將評論標記為垃圾。 |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | 版主頒發徽章給使用者。 |

### Multiple triggers per agent

代理可以訂閱任意組合的觸發器 — 例如，[Moderator template](#template-moderator) 同時訂閱 `COMMENT_ADD` 與 `COMMENT_FLAG_THRESHOLD`。每個事件會在該事件的上下文中觸發代理一次。

### What stops an agent firing

已訂閱的觸發事件若符合下列任一情況，將**不**會觸發代理：

- 代理的 [status](#status-states) 為 **已停用**。
- 代理的 [URL 或語系範圍](#scope-url-locale) 與觸發評論不符。
- 代理的 [每日、每月或速率限制預算](#budgets-overview) 已耗盡 — 該觸發會被記錄為 **已丟棄** 並附上原因。請參見 [Drop Reasons](#drop-reasons)。
- 該代理的併發數已飽和（以每代理上限為準）。
- 該代理所屬的租戶帳單無效。
- 觸發該動作的實體本身是機器人或另一個代理（用於防止迴圈）。
- 觸發是針對在去重視窗內已被此代理處理過的評論。

當已訂閱的觸發成功執行時，代理的 [Run History](#run-history) 會顯示一列狀態為 **已啟動** 的紀錄，執行完成後會轉為 **成功** 或 **錯誤**。

### Vote and flag thresholds

兩個觸發 — **Comment Crosses Vote Threshold** 與 **Comment Crosses Flag Threshold** — 在編輯表單上需要設定數值門檻。觸發會在計數跨過設定值的那一刻發生（具體而言，flag-threshold 觸發在 `flagCount === flagThreshold` 時發生，因此選 1 表示「在第一個檢舉時觸發」，選 5 表示「在第五個檢舉到達時觸發」）。

### Deferred triggers

任何觸發都可以被延後執行，讓代理稍後運行，例如在投票/檢舉/回覆有時間趨於穩定後。請參見 [Deferred Triggers](#trigger-deferred-delay)。

### Loop prevention

為了防止無限迴圈，由代理**撰寫的評論** 會帶有 `botId`。新評論觸發會忽略帶有 `botId` 的評論。

淨效應是：代理可以對租戶中的 *人類* 動作作出反應，但由代理產生的動作永遠不會觸發任何代理觸發器。這適用於所有觸發類型。

### REPLAY: the internal trigger

系統內部也有一個用於 [Test Runs (Replays)](#test-runs-replays) 功能的 `REPLAY` 觸發類型。您無法在編輯表單上選擇它 — 它存在的目的是讓重放執行在執行記錄中有明確標記，且在即時執行檢視中被排除。