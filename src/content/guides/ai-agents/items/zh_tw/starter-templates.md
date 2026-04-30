FastComments 提供五個啟動範本，讓你不必從零開始撰寫可運作的代理。你可以從 [AI Agents 頁面](https://fastcomments.com/auth/my-account/ai-agents) 點選 **Browse templates** 即可取得這些範本。

當你選擇一個範本時：

1. 代理會以 **狀態：試運行 (Dry Run)** 建立，並以範本為基礎命名內部名稱（`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`）。如果該名稱在你的租戶中已被使用，系統會加上數字後綴。
2. 你會直接進入編輯表單，所有項目都會預先填好 — 提示詞、觸發條件、允許的操作，以及任何閾值。頂端會有一個橫幅顯示「Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready.」。
3. 目前尚未啟用。代理在你儲存並且要麼保持試運行（觀察用）、要麼切換為已啟用之前都不會執行任何動作。

### 這五個範本

- **[版主](#template-moderator)** - 審查新評論與被檢舉的評論，對初犯者發出警告，僅在警告後才升級為封禁。會在新評論以及達到檢舉閾值時觸發（預設 flag threshold: 3）。允許的工具：`mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`。

- **[歡迎迎賓](#template-welcome-greeter)** - 向首次留言的使用者以簡短且有溫度的個人化歡迎回覆。觸發條件為 new-user-first-comment。允許的工具：`write_comment`。

- **[熱門評論釘選器](#template-top-comment-pinner)** - 在實質性頂層評論達到票數閾值後將其釘選（預設：10），並先取消先前被釘選的評論。觸發條件為 vote-threshold crossings。允許的工具：`pin_comment`, `unpin_comment`。

- **[討論串摘要器](#template-thread-summarizer)** - 在長討論串延遲一段時間後發表中性、一段落的摘要，然後將其釘選。觸發條件為新評論，並延遲 30 分鐘以便討論串穩定後再摘要。允許的工具：`write_comment`, `pin_comment`, `unpin_comment`。

- **[Gaslight 偵測器](#template-gaslight-detector)** - 監視評論編輯以發現會扭曲回覆的討論串中途改寫，回復原始文字並私訊作者。觸發條件為評論編輯。允許的工具：`edit_comment`, `warn_user`, `send_dm`。

### 自訂範本

範本只是起點，而非合約。你應該：

- 微調 **Initial prompt** 以符合你的社群語氣。
- 新增或移除 **Triggers**，以配合你希望代理運作的頻率。
- 為任何敏感操作加入 **Approvals** — 我們強烈建議對於類似版主的範本將 `ban_user` 設為需經審核。
- 新增 **Community guidelines**，讓代理能一致地套用你書面的政策。參見 [社群準則](#community-guidelines)。
- 根據你預期的觸發次數，為每個代理設定適當的 **預算 (Budgets)**。

範本只是預填合理預設值的工具；一旦儲存，該代理就屬於你。