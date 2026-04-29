**Template ID:** `welcome_greeter`

歡迎迎賓器會以熱情的方式回覆首次留言者。這是風險最低的範本（沒有破壞性工具），也是一個適合首發上線的代理人。

### 內建初始提示

[inline-code-attrs-start title = '歡迎迎賓範本初始提示'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### 觸發條件

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

此事件對每個使用者只會觸發一次，因此代理人無法迴圈重複執行。詳見 [Trigger: New User First Comment](#trigger-new-user-first-comment)。

### 允許的工具

- [`write_comment`](#tools-overview)

這是唯一可用的工具——代理人實際上無法進行審核、投票、封鎖或私訊。

### 上線前的建議補充

- **Set the Display name** to something inviting - "Community Bot", your site mascot, or your brand name. The display name is what readers see attached to the welcome reply.
- **Tick "Include page title, subtitle, description, and meta tags"** in [Context Options](#context-options). The greeter's replies become noticeably better when it can reference what the page is actually about.
- **Consider locale restrictions** if you operate in multiple languages. A welcome reply in the wrong language is more jarring than a missed reply. See [Scope: URL and Locale Filters](#scope-url-locale).

### 為何不需要審批

此代理人僅撰寫新留言，且僅在一次性觸發時執行。最糟情況也只是一次尷尬的問候。沒有需要把關的破壞性行為。大多數運營者在測試運行（dry-run）看起來正常後，便完全不設審批就運行它。

---