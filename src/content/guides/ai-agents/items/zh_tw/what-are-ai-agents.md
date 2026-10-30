---
**AI Agent** 是一個自主的工作者，受限於您的 FastComments 租戶，會監控社群中的事件並代表您採取行動。

每個代理都有三項您可以控制的內容：

1. **個性。** 一段自由文字的初始提示，定義語氣、角色與決策風格（「您是熱情的社群迎接者」、「您執行社群規則，但傾向以警告而非封禁」等）。
2. **一個或多個觸發條件。** 一系列會喚醒代理的事件——新評論、評論達到投票或檢舉門檻、版主操作、使用者在網站上的首次評論等。完整列表請參見 [Trigger Events Overview](#triggers-overview)。
3. **工具白名單。** 代理被允許執行的操作——發表評論、投票、置頂、鎖定、標記為垃圾訊息、封禁使用者、透過私訊警告、頒發徽章、發送電子郵件、儲存與搜尋共享記憶。完整列表請參見 [Allowed Tool Calls Overview](#tools-overview)。

當觸發條件被觸發時，代理會收到描述發生情況的上下文訊息（評論、頁面，以及可選的討論串/使用者/頁面上下文），並以其初始提示與您的社群指南作為提示。接著它會呼叫工具執行操作，並為每一次呼叫記錄理由與信心分數。

### 代理以非同步方式執行

代理 **永不阻塞觸發它們的使用者操作**。讀者發表評論，評論被儲存並顯示在討論串中，回應被返回，只有在*此時*代理才會對其執行——可以立即執行或在設定的延遲之後（請參見 [Deferred Triggers](#trigger-deferred-delay)）。代理所做的任何事都不會增加使用者端的延遲。

### 為什麼使用它們

- **大規模審核。** 標記明顯的垃圾訊息並封禁重犯者，無需全天候監控審核隊列。
- **歡迎新評論者。** 以您的語氣回覆首次評論的使用者。
- **凸顯最佳內容。** 當實質性的頂層評論達到投票門檻時即置頂。
- **一致執行您的指南。** 對每一則邊緣評論套用相同的政策文字。
- **摘要長篇討論串。** 發布多頁辯論的中立摘要。

### 讓您保持掌控的因素

- **乾跑模式。** 每個新代理皆以 **Dry Run** 方式出貨：它會處理觸發條件、執行模型，並記錄它 *本會* 做的事，但不會執行任何實際操作。請參見 [Dry-Run Mode](#dry-run-mode)。
- **批准流程。** 任意子集的操作都可以設置為需要人工批准。請參見 [Approval Workflow](#approval-workflow)。
- **每代理與每帳戶的預算。** 設有嚴格的每日與每月上限。請參見 [Budgets Overview](#budgets-overview)。
- **工具白名單。** 不允許的工具會從模型的調色盤中剔除——代理根本無法請求它們。請參見 [Allowed Tool Calls Overview](#tools-overview)。
- **每項操作的稽核欄位。** 模型必須包含理由與信心分數。兩者皆會出現在執行時間軸與每次批准中。請參見 [Run Detail View](#run-detail-view)。
- **EU DSA 第 17 條。** 在歐盟地區，完全自動化的封禁會被阻止。請參見 [EU DSA Article 17 Compliance](#eu-dsa-compliance)。
- **不使用您的資料進行訓練。** FastComments 使用的供應商不會以您的提示或評論作為訓練資料。

### 它們在人類審核中的定位

代理與人類版主共享相同的評論平台：代理透過相同的渠道（批准、垃圾訊息、封禁、徽章、置頂、鎖定、寫入）執行操作，且這些操作會出現在相同的 [Comment Logs](/guide-moderation.html#comment-logs)、相同的 [Moderation Page](/guide-moderation.html#moderate-comments-page) 以及相同的通知串流中。代理與人類都能看到彼此的工作，並可相互回應——版主的操作本身也是有效的代理觸發條件（請參見 [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) 等）。

如果您也需要人力處理隊列，FastComments 提供 [Human Moderation Services](/guide-moderation.html#human-moderation-services)：我們的版主會與您的代理一起工作，顯示於您的儀表板中，並遵循您的指南。
---