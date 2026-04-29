**Template ID:** `tos_enforcer`

若您的目標是減少人工審核負擔，建議以版主範本為起點。它會審查新發表和被標記的留言，並套用您的社群規則。

### 內建初始提示

[inline-code-attrs-start title = '版主範本初始提示'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

您幾乎總是會想要以具體範例來**擴充此提示**，說明您網站允許與不允許的內容。平台本身的升級政策（在封鎖前先警告、在封鎖前搜索記憶）已經內置在 agent 接收到的系統提示中，因此您不需重複說明。

### 觸發條件

- **New comment posted** (`COMMENT_ADD`) - agent 會檢視每則新留言。
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - agent 會重新評估被其他使用者標記的留言。

### 可用工具

- [`mark_comment_approved`](#tools-overview) - 對採用預審模式的租戶很有用，agent 會放行乾淨的留言並隱藏其餘留言。
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

它無法張貼留言、投票、置頂、鎖定、頒發徽章或發送電子郵件——該提示刻意縮小了權限範圍。

### 上線前建議新增項目

- **Set [Community Guidelines](#community-guidelines).** 幾句書面政策即可；agent 每次執行時都會套用。
- **Gate `ban_user` behind [approval](#approval-workflow).** 此功能在歐盟地區預設為啟用（參見 [EU DSA Article 17 Compliance](#eu-dsa-compliance)），並建議在各地區採用。
- **Consider also gating `mark_comment_spam` behind approval**，若您的流量低但內容風險高，建議這樣做。
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** 核准一則不當留言會讓它出現在讀者面前；在 agent 透過試跑建立信任前，應先限制此權限。
- **勾選「包括留言者的信任指數、帳號年齡、被封禁紀錄與近期留言」** 在 [Context Options](#context-options)。當模型能看到某人是長期誠信使用者時，會較不激進地發出警告。

### 建議的試跑期間

在轉為啟用前，請至少以真實流量將此範本在 [dry-run](#dry-run-mode) 模式下執行一週。使用 [Test Runs (Replays)](#test-runs-replays) 同時預覽過去 30 天的情況。

---