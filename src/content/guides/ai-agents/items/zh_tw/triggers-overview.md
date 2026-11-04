**觸發器（trigger）**是喚醒代理（agent）的事件。每個代理可以定義一個或多個觸發器。

### 完整清單

| 觸發器 | 觸發時機 |
|---|---|
| [Comment Added](#trigger-comment-add) | 有新留言發佈。 |
| [Comment Edited](#trigger-comment-edit) | 留言被編輯。先前的文字會包含在代理的上下文中。 |
| [Comment Deleted](#trigger-comment-delete) | 留言被刪除。 |
| [Comment Pinned](#trigger-comment-pin) | 留言被釘選（由任何人，包括版主或其他代理）。 |
| [Comment Unpinned](#trigger-comment-unpin) | 留言被取消釘選。 |
| [Comment Locked](#trigger-comment-lock) | 留言被鎖定（不允許再回覆）。 |
| [Comment Unlocked](#trigger-comment-unlock) | 留言被解鎖。 |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | 留言的淨票數達到設定的門檻。 |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | 留言的檢舉數恰好達到設定的門檻。 |
| [User Posts First Comment](#trigger-new-user-first-comment) | 使用者在這個網站上發佈他們的第一則留言。 |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | 留言被垃圾訊息引擎自動標記為垃圾訊息。 |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | 版主將留言標記為已審核。 |
| [Moderator Approves Comment](#trigger-moderator-approved) | 版主核准一則留言。 |
| [Moderator Marks Spam](#trigger-moderator-spammed) | 版主將留言標記為垃圾訊息。 |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | 版主頒發徽章給使用者。 |

### 每個代理可有多個觸發器

一個代理可以訂閱任意組合的觸發器——例如 [Moderator 範本](#template-moderator) 同時訂閱了 `COMMENT_ADD` 和 `COMMENT_FLAG_THRESHOLD`。每個事件都會以該事件的上下文觸發代理一次。

### 什麼情況下代理不會被觸發

在以下任一情況下，已訂閱的觸發器事件**不會**觸發代理：

- 代理的[狀態](#status-states)為 **Disabled**。
- 代理的 [URL 或語系範圍](#scope-url-locale)與觸發的留言不符。
- 代理的[每日、每月或速率限制預算](#budgets-overview)已用盡——該觸發器會以原因記錄為 **dropped**。請參閱 [Drop Reasons](#drop-reasons)。
- 此代理的並發數已飽和（每個代理有上限）。
- 代理所屬租戶的帳務無效。
- 觸發動作本身是由機器人或其他代理執行的（防止循環）。
- 該觸發器針對的留言已在去重複視窗內被此代理處理過。

當已訂閱的觸發器成功觸發時，代理的 [Run History](#run-history) 會顯示一筆狀態為 **Started** 的記錄，並在執行完成後轉為 **Success** 或 **Error**。

### 票數與檢舉門檻

兩個觸發器——**Comment Crosses Vote Threshold** 和 **Comment Crosses Flag Threshold**——需要在編輯表單上設定數值門檻。當計數超過設定值的那一刻，觸發器就會觸發（具體來說，檢舉門檻觸發器是在 `flagCount === flagThreshold` 時觸發，因此設定 1 表示「第一次被檢舉時觸發」，設定 5 表示「第五次檢舉到達時觸發」）。

### 延遲觸發器

任何觸發器都可以設定延遲，讓代理稍後才執行，例如在票數/檢舉/回覆有時間穩定之後。請參閱 [Deferred Triggers](#trigger-deferred-delay)。

### 防止循環

為了防止無限循環，**由代理撰寫的**留言會帶有 `botId`。新留言觸發器會忽略帶有 `botId` 的留言。

最終效果：代理可以回應您租戶中*人類*的動作，但代理來源的動作永遠不會觸發任何代理觸發器。這適用於所有觸發器類型。

### REPLAY：內部觸發器

還有一個內部的 `REPLAY` 觸發器類型，由 [Test Runs (Replays)](#test-runs-replays) 功能使用。您無法在編輯表單上選擇它——它的存在是為了讓重播執行在執行歷史中被明確標記，並從即時執行檢視中排除。