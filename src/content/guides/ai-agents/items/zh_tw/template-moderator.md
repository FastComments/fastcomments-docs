**範本 ID:** `tos_enforcer`

Moderator 範本是建議的起點，如果你的目標是減少人工審核負擔。它會檢視新的及被標記的留言並套用你的社群規則。

你幾乎在所有情況下都會想要**擴充內建提示**，加入具體的範例說明你的網站允許與不允許的內容。平台本身的升級政策（在封鎖前先警告、在封鎖前搜尋記憶）已內建在代理程式收到的系統提示中，因此你不需要重複它。

### 觸發條件

- **New comment posted** (`COMMENT_ADD`) - 代理程式會檢視每則新留言。
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, 預設門檻：3) - 代理程式會重新評估其他使用者已標記的留言。

### 允許的工具

- [`mark_comment_approved`](#tools-overview) - 對於採取預審模式的租戶很有用，代理程式會放行乾淨的留言並隱藏其餘的。
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

它無法發表留言、投票、置頂、鎖定、頒發徽章或發送電子郵件 — 提示刻意設計得功能有限。

### 上線前建議新增項目

- **設定 [Community Guidelines](#community-guidelines)。** 幾句文字化的政策就足夠；代理程式會在每次執行時套用它。
- **將 `ban_user` 設為需經 [審核](#approval-workflow) 才可執行。** 在歐盟區域預設為開啟（參見 [EU DSA Article 17 Compliance](#eu-dsa-compliance)），並建議在所有地區啟用。
- **若你的內容量低但風險高，亦可考慮將 `mark_comment_spam` 設為需經審核。**
- **若你採用預審，將 `mark_comment_approved` 設為需經審核。** 核准一則不當留言會讓它出現在讀者面前；在代理程式透過試運行建立信任之前，請將其設為需審核。
- **在 [Context Options](#context-options) 勾選「包含留言者的信任指數、帳號年資、被封鎖紀錄與近期留言」。** 當模型能看到某人是長期且善意的使用者時，會較不積極地發出警告。

### 建議的試運行時段

在將此範本切換為啟用前，先以 [dry-run](#dry-run-mode) 模式對實際流量執行至少一週。也可使用 [Test Runs (Replays)](#test-runs-replays) 來預覽過去 30 天的結果。

---