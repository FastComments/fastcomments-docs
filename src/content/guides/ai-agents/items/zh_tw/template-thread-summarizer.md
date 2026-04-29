**Template ID:** `thread_summarizer`

Thread Summarizer 會在長篇討論串結尾發布中性、單段落的摘要。它使用 30 分鐘的延遲，以便討論串穩定後再由代理檢視。

內建提示指示代理不要加入編輯性意見——這一點很重要。若沒有它，模型會傾向使用「在我看來」之類的表述，這在以您的帳戶顯示名稱發布時讀起來效果很差。

### Triggers

- **New comment posted** (`COMMENT_ADD`)。
- **Trigger delay**: 30 minutes (1800 seconds)。參閱 [Deferred Triggers](#trigger-deferred-delay)。

30 分鐘的延遲表示代理在評論發佈後半小時執行一次，根據當時討論串的狀態進行摘要。這並非「對每則回覆都摘要」——延遲觸發佇列會將同一討論串上的多個新評論事件合併，但不會跨不同時間窗去重複它們。您可能會想在提示中**加入自訂規則**，例如「如果代理在過去 24 小時內已經為此討論串撰寫過摘要，則不要發佈新的摘要」，並依靠上下文以及代理的 [memory tools](#tools-overview) 來執行該規則。

### Allowed tools

- [`write_comment`](#tools-overview) - 發佈摘要本身。
- [`pin_comment`](#tools-overview) - 將摘要釘選，讓讀者在討論串頂端看到它。
- [`unpin_comment`](#tools-overview) - 在釘選新摘要之前，取消釘選該代理先前的摘要。

摘要器無法進行審核或與使用者互動。

### Pinning the summary

代理使用 `write_comment` 發佈新評論，然後以回傳的評論 ID 呼叫 `pin_comment`。在對同一討論串的後續執行中，提示指示代理先對其先前的摘要呼叫 `unpin_comment`——平台本身並不強制每個討論串只能有一則釘選評論，因此如果保留先前摘要的釘選，會導致兩則針對同一討論串的釘選摘要並列出現。於 [Context Options](#context-options) 中勾選「Include parent comment and prior replies in the same thread」，以便代理能看到先前已釘選的摘要。

### Recommended additions before going live

- **在 [Context Options](#context-options) 勾選「Include parent comment and prior replies in the same thread」**。沒有討論串上下文的摘要器毫無用處。
- **調整最低討論串大小規則。** 提示預設為「Fewer than 5 comments」，但在活躍社群中 10–20 則較為合適。直接編輯提示。
- **限制特定 URL 模式**，如果您只想在長文頁面上產生摘要，而非在公告或產品頁面上。參閱 [Scope: URL and Locale Filters](#scope-url-locale)。
- **注意成本。** 摘要模板是最耗費 token 的，因為每次執行都會讀取整個討論串。在切換為啟用之前，先設定一個嚴格的 [daily budget](#budgets-overview)。

### Avoiding repeat summaries

代理可以使用 [`save_memory`](#tools-overview) 和 [`search_memory`](#tools-overview)——您可以擴充提示，指示代理記錄「summarized {thread urlId}」的備註，並在再次發佈前檢查這些備註。記憶在您租戶中的所有代理之間共享。