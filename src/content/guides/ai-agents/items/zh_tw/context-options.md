The **Context** 區段位於編輯表單上，控制代理每次執行時收到多少資訊。更多的上下文會產生更好的決策，但會提高每次執行的 token 成本，因此你只應提供代理實際需要的資訊。

### What's always included

即使所有核取方塊都未勾選，代理的上下文訊息仍會包含：

- The **trigger event type** (例如 `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`)。
- The **page URL and URL ID**（若已知）。
- 觸發執行的 **comment**（若有）— ID、作者使用者 ID、作者顯示名稱、留言文字、投票數、檢舉數、垃圾郵件/通過/已審查 標記、父項 ID。作者的電子郵件**絕不**會傳送給 LLM 提供者（以降低個資風險）。
- `COMMENT_EDIT` 觸發時的 **先前留言文字**（以便代理比較編輯前後）。
- `COMMENT_VOTE_THRESHOLD` 觸發時的 **投票方向**。
- 觸發者的 **使用者 ID** 及 **徽章 ID**（用於版主徽章觸發）。

所有不受信任的文字——留言內容、作者名稱、頁面標題、指南文件本身——在上下文訊息中都以像 `<<<COMMENT_TEXT>>> ... <<<END>>>` 這樣的標記被「圍起來」（fenced）。平台的系統提示指示模型絕不跟隨那些囲起來內容內的指示。這是平台的提示注入（prompt-injection）防護；你不需要在你的提示中重複它。

### The three checkboxes

#### Include parent comment and prior replies in the same thread

會新增：
- **parent comment** — ID、作者、文字。
- **Sibling replies** — 同一討論串中對同一父留言的先前回覆。

用途：適用於任何需要在上下文中回應留言的代理（歡迎致詞、自動回覆串摘要、閱讀對話中的回覆之版主）。

成本：小到中等。受限於該討論串中有多少個兄弟回覆。

#### Include commenter's trust factor, account age, ban history, and recent comments

會新增 **AUTHOR_HISTORY** 區塊：

- **Account age in days** 自註冊以來的天數。
- **Trust factor (0-100)** — FastComments 的分數，用以總結該使用者在此站點上的信任程度。參見 [垃圾郵件偵測](/guide-moderation.html#spam-detection) 頁面（在審核指南中）。
- **Prior ban count.**
- **Total comments on this site.**
- **Duplicate-content count** — 若該使用者最近曾張貼相同文字（反垃圾訊號）。
- **Same-IP cross-account signal** — 從相同 IP 在其他帳號下張貼留言的次數（多帳號訊號）。IP 雜湊本身絕不會傳送給 LLM。
- **Recent comments** — 該使用者最多最近 5 則留言，每則最多截斷為 300 字元，並以不受信任文字的方式圍起來。

用途：適用於任何審核代理。沒有這些資料時，模型可能會對新帳號以及長期誠信良好的使用者採取相同的封鎖態度。

成本：中等。最近的留言會增加最多的 token 數量。

#### Include page title, subtitle, description, and meta tags

會新增 **PAGE_CONTEXT** 區塊 — 標題、副標題、描述，以及 FastComments 為該頁面擷取的任何 meta 標籤。

用途：適用於歡迎致詞和討論串摘要器，在知道頁面主題時能大幅提升輸出品質。

成本：小。

### Community guidelines

第四個欄位，**Community guidelines**，是一個自由文字的政策區塊，會在每次執行時包含在使用者角色的上下文訊息中，並以與留言內容和其他使用者提供內容相同的方式被圍起來為不受信任文字。代理將其視為政策文字閱讀，但平台不會將其視為系統指令。關於應放入內容的範例，請參閱 [Community Guidelines](#community-guidelines)。

### Adding context selectively

這些核取方塊是針對每個代理設定，而非全域設定。一個常見的配置範例：

- 歡迎致詞：頁面上下文 **開啟**、討論串上下文 **關閉**、使用者歷史 **關閉**。
- 版主：討論串上下文 **關閉**、使用者歷史 **開啟**、頁面上下文 **關閉**。
- 討論串摘要器：討論串上下文 **開啟**、頁面上下文 **開啟**、使用者歷史 **關閉**。

在代理實際呼叫時能正確運作所需的最少上下文即可——額外的上下文會在每次執行時增加 token 成本，即便該代理並未使用那些額外的資訊。