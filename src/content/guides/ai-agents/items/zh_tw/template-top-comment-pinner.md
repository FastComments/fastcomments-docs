---
**範本 ID：** `top_comment_pinner`

置頂留言釘選器會監視達到票數門檻的頂層留言並將其釘選——替換同一討論串上先前釘選的任何內容。

### 內建初始提示

[inline-code-attrs-start title = '置頂留言釘選器範本初始提示'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

「不要釘選回覆」這項指示很重要：釘選是針對討論串運作，所以釘選回覆通常沒什麼用。「非促銷性」的過濾可以防止代理推廣受歡迎的連結垃圾留言。

### 觸發條件

- **留言達到票數門檻** (`COMMENT_VOTE_THRESHOLD`，預設票數門檻：10)。

當留言的淨得票數（`up - down`）達到設定的門檻時，觸發器會啟動。根據你的討論串活躍程度在編輯表單上調整數值——對於中等活躍的網站來說，10 是合理的預設值。

### 允許的工具

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

釘選為非破壞性操作—可以立即還原—因此此範本通常在不需審核的情況下執行。

### 上線前建議加入

- **勾選「包含父留言及同一討論串中先前的回覆」** 在 [上下文選項](#context-options)。沒有討論串上下文，代理無法可靠判斷是否已有釘選留言需要取消釘選。
- **調整票數門檻** 以符合你的網站。在熱門討論串上 10 太常發生；在冷門討論串上 10 可能永遠不會達到。
- **考慮以 URL 進行範圍限定**，如果你只想在網站的某些區段顯示釘選留言——例如新聞討論串，但不是公告討論串。

### 關於重複釘選的注意事項

代理的提示指示它在釘選之前先取消先前的釘選，但如果模型漏掉那一步，平台本身並不強制每個討論串只能有一個釘選（你可以有多個）。如果重複釘選在你的網站上造成問題，將 `pin_comment` 設為需審核並逐一檢查—或撰寫更嚴謹的提示。

---