FastComments 提供四個入門範本，讓你不必從頭撰寫可運作的代理。你可以在 [AI 代理頁面](https://fastcomments.com/auth/my-account/ai-agents) 上點選 **瀏覽範本** 來存取它們。

當你選擇一個範本時：

1. 代理會以 **狀態：測試執行** 建立，內部名稱基於該範本（`tos_enforcer`、`welcome_greeter`、`top_comment_pinner`、`thread_summarizer`）。如果該名稱在你的租戶中已被使用，將加入數字後綴。
2. 你會直接進入編輯表單，所有欄位皆已預先填好 — 提示、觸發條件、允許的動作與任何門檻值。頂部橫幅顯示「從 {templateName} 範本建立。請檢閱下方設定，準備好時將狀態改為已啟用。」
3. 目前尚未啟用。代理在你儲存且要麼保持測試模式（以觀察），要麼切換為已啟用之前，均不會執行任何動作。

### 四個範本

- **[Moderator](#template-moderator)** - 審查新留言與被檢舉的留言，對初犯者給予警告，僅在警告後才升級為禁止。觸發於新留言與達到檢舉門檻時（預設檢舉門檻：3）。允許工具：`mark_comment_approved`、`mark_comment_spam`、`warn_user`、`ban_user`。

- **[Welcome Greeter](#template-welcome-greeter)** - 對首次留言的使用者以簡短、具個人色彩的歡迎回覆。觸發於 new-user-first-comment。允許工具：`write_comment`。

- **[Top Comment Pinner](#template-top-comment-pinner)** - 當具有實質內容的頂層留言達到票選門檻（預設：10）時將其釘選，且會先解除先前被釘選的留言。觸發於票選門檻達成時。允許工具：`pin_comment`、`unpin_comment`。

- **[Thread Summarizer](#template-thread-summarizer)** - 在討論串延遲一段時間後，發佈中立的單段摘要，然後將其釘選。觸發於新留言，並有 30 分鐘的延遲以便討論串穩定後再摘要。允許工具：`write_comment`、`pin_comment`、`unpin_comment`。

### 自訂範本

範本只是起點，而非合約。建議你：

- 調整 **初始提示** 以符合你的社群語氣。
- 新增或移除 **觸發條件**，以控制代理執行頻率。
- 為任何敏感操作新增 **核准** — 我們強烈建議在版主型範本中，將 `ban_user` 設為需經核准。
- 新增 **社群指南**，讓代理能一致地套用你書面的政策。參見 [Community Guidelines](#community-guidelines)。
- 為每個代理設定適當的 **預算**，根據你預期的觸發次數來調整。

範本只是預先填入合理預設值的工具；一旦儲存，該代理即屬於你。