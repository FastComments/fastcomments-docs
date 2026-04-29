**Template ID:** `top_comment_pinner`

Top Comment Pinner 會監視達到票數門檻的頂層留言並將其置頂 - 取代同一討論串先前被置頂的留言。

內建提示指示代理跳過回覆（置頂在討論串層級運作，所以置頂回覆通常沒什麼用）並過濾促銷內容（以免代理推高受歡迎的連結垃圾訊息）。

### Triggers

- **A comment crosses a vote threshold** (`COMMENT_VOTE_THRESHOLD`, default vote threshold: 10).

當留言的淨票數（`up - down`）達到設定的門檻時，觸發器會啟動。根據討論串的活躍度，在編輯表單中調整該數值 - 對於中等活躍的網站，10 是合理的預設值。

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

置頂操作是非破壞性的 - 可立即還原 - 因此此範本通常在不需審核的情況下執行。

### Recommended additions before going live

- **勾選「包含同一討論串中的父留言與先前回覆」** 在 [Context Options](#context-options)。沒有討論串內容時，代理無法可靠判斷是否已經有要取消置頂的留言。
- **調整票數門檻** 以符合您的網站。在熱門討論串中，10 可能太常觸發；在冷清討論串中，10 可能永遠不會達成。
- **考慮依 URL 範圍限定** 如果您只想在網站的特定區段顯示置頂留言 - 例如新聞討論串，但非公告討論串。

### Note on duplicate pinning

代理的提示會指示它在置頂前先取消先前的置頂，但如果模型漏掉那個步驟，平台本身並不強制每個討論串只能有一則置頂（可以有多則）。如果重複置頂在您的網站上造成問題，請將 `pin_comment` 設為需審核後才能執行並檢查每一則 - 或撰寫更嚴謹的提示。