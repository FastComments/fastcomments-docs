在 [AI Agents 頁面](https://fastcomments.com/auth/my-account/ai-agents) 中，你可以用兩種方式建立代理：

- **從範本建立。** 點選 **Browse templates** 並選擇四個內建啟動代理之一。表單會預先填好，代理的狀態為 **Dry Run**。參見 [Starter Templates](#starter-templates)。
- **從頭開始。** 點選 **Create new agent**。表單為空白。

不論哪一種方式，儲存後以及之後編輯的都是相同的編輯表單。本頁將從表單最上方開始逐項說明。

### 基礎

- **Internal name.** 僅在管理儀表板（執行歷史、分析、稽核日誌）中使用的短識別字串。小寫並用底線效果良好： `moderator`, `welcome_greeter`。如果範本的 internal name 已被使用，表單會自動加後綴（`tos_enforcer_2` 等）。
- **Display name.** 當代理發表評論時公開顯示的名稱。這是讀者會看到的名稱。
- **Status.** 停用、模擬執行或啟用。新代理預設為模擬執行。參見 [Status States](#status-states)。

### 模型

選擇 LLM。參見 [Choosing a Model](#choosing-a-model)。

### 預算

可選的每日與每月上限（使用您帳戶的貨幣），以及一個 **Alert thresholds** 勾選清單（預設 80% 與 100%）。參見 [Budgets Overview](#budgets-overview) 與 [Budget Alerts](#budget-alerts)。

### 個性

**Initial prompt** 是定義語氣、角色與決策規則的 system prompt。純文字，不可使用範本語法。參見 [Personality and the Initial Prompt](#personality-prompt)。

### 上下文

Context 欄位集包含三個核取方塊、一個指導方針文字區，以及範圍輸入欄位：

- 在相同串流中包含父評論與先前回覆。
- 包含評論者的信任因子、帳號年資、封鎖歷史與近期評論。
- 包含頁面標題、副標題、描述與 meta 標籤。
- 一個可選的 **Community guidelines** 文字區塊，會被置於每個 prompt 的前段。
- **Restrict to specific pages** - URL 模式允許清單（每行一個）。留空表示適用於整個租戶。
- **Restrict to specific locales** - 使用雙欄清單選取器的語系允許清單。留空表示適用於所有語系。

更多上下文會提升決策品質，但會提高每次執行的 token 成本。參見 [Context Options](#context-options)、[Community Guidelines](#community-guidelines) 與 [Scope: URL and Locale Filters](#scope-url-locale)。

### 觸發器

從清單中至少選一個事件。對於 vote-threshold 與 flag-threshold 觸發器，你也必須設定閾值。可選的 **Delay before running** 欄位會在觸發器觸發後延遲執行（對於票數尚在穩定的檢舉閾值情形特別有用）。參見 [Trigger Events Overview](#triggers-overview) 與 [Deferred Triggers](#trigger-deferred-delay)。

### 允許的工具呼叫

勾選 **Allow any tool calls** 以開放完整的工具面板。否則請勾選允許代理使用的特定工具 — 不被允許的工具會從模型的工具面板中移除，並在派遣時被拒絕。**Ban options** 子節將破壞性封鎖變體（delete-all-comments, ban-by-IP）放在明確選擇的門檻後。參見 [Allowed Tool Calls Overview](#tools-overview) 與 [Tool: ban_user](#tool-ban-user)。

### 審核/批准

勾選必須由人工審核通過才能由代理執行的操作。批准僅適用於代理被允許呼叫的工具；未允許的工具會被直接拒絕。在歐盟區域，根據數位服務法第 17 條，**ban_user** 項目為強制開啟。參見 [Approval Workflow](#approval-workflow) 與 [EU DSA Article 17 Compliance](#eu-dsa-compliance)。

### 批准通知

若啟用批准，選擇要收到電子郵件的人員：

- **All admins and moderators** - 帳戶擁有者、超級管理員與評論版主管理員。
- **Specific users** - 從雙欄清單選取器中手動挑選。

每位審查者的個別傳送頻率（立即、每小時彙整、每日彙整）在其個人設定檔中設定。參見 [Approval Notifications](#approval-notifications)。

### 統計

唯讀。包含總執行次數、最後執行時間戳記，以及代理最近發表的評論 ID（若有）。

### 儲存

點選 **Save agent**。頁面會導回代理清單。新代理在模擬執行狀態下立即有資格接受觸發。

### 之後編輯

代理清單頁面的每個列都會顯示個別代理操作：**Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, 與 **Delete**。編輯代理不會回溯影響已記錄的執行紀錄 — 歷史會被保留。Replay 快照也會在開始 replay 時凍結代理的設定，因此已儲存的 replay 結果即使在你編輯 prompt 後仍可重現。