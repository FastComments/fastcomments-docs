FastComments 提供五個入門範本，讓您不必從零開始撰寫可運作的代理。您可以從 [AI Agents 頁面](https://fastcomments.com/auth/my-account/ai-agents) 點擊 **Browse templates** 來存取這些範本。

當您選擇一個範本時：

1. 代理會以 **Status: Dry Run** 建立並使用基於範本的內部名稱（`tos_enforcer`、`welcome_greeter`、`top_comment_pinner`、`thread_summarizer`、`gaslight_detector`）。如果該名稱在您的租戶上已被使用，則會加上數字後綴。
2. 您會直接進入已預填所有內容的編輯表單 - 提示詞、觸發器、允許的操作以及任何閾值。頂部的橫幅會顯示「Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready.」
3. 此時尚未啟用任何功能。在您儲存並保持乾跑模式（以便觀察）或切換為 Enabled 之前，代理不會執行任何操作。

### 五個範本

- **[Moderator](#template-moderator)** - 審核新留言與被標記的留言，警告初犯者，僅在警告後才升級為封禁。在新留言以及標記閾值跨越時觸發（預設標記閾值：3）。允許的工具：`mark_comment_approved`、`mark_comment_spam`、`warn_user`、`ban_user`。

- **[Welcome Greeter](#template-welcome-greeter)** - 以簡短、個人化的歡迎訊息熱情回覆首次留言者。在新使用者的首次留言時觸發。允許的工具：`write_comment`。

- **[Top Comment Pinner](#template-top-comment-pinner)** - 當實質性的頂層留言跨越投票閾值（預設：10）時將其置頂，並先取消先前置頂的留言。在投票閾值跨越時觸發。允許的工具：`pin_comment`、`unpin_comment`。

- **[Thread Summarizer](#template-thread-summarizer)** - 在延遲一段時間後，於長討論串上發布中立的單段摘要，然後將其置頂。在新留言時觸發，並有 30 分鐘的延遲，讓討論串在摘要前先沉澱。允許的工具：`write_comment`、`pin_comment`、`unpin_comment`。

- **[Gaslight Detector](#template-gaslight-detector)** - 監控留言編輯，偵測扭曲回覆的討論串中途改寫，還原原始文字，並傳送私訊給作者。在留言編輯時觸發。允許的工具：`edit_comment`、`warn_user`、`send_dm`。

### 自訂範本

範本是起點，而非契約。您應該：

- 調整 **Initial prompt** 以符合您社群的語氣。
- 新增或移除 **Triggers** 以符合代理應執行的頻率。
- 為任何敏感操作新增 **Approvals** - 我們強烈建議在主持人風格的範本中，將 `ban_user` 置於審批機制之後。
- 新增 **Community guidelines**，讓代理一致地套用您的書面政策。請參閱 [Community Guidelines](#community-guidelines)。
- 根據您預期的觸發次數，設定每個代理適當的 **Budgets**。

範本只是預填合理預設值的載體；一旦儲存，代理就屬於您了。