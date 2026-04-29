**模板 ID：** `welcome_greeter`

The Welcome Greeter replies warmly to first-time commenters. It is the lowest-risk template (no destructive tools) and a good first agent to ship live.

### 内置初始提示

[inline-code-attrs-start title = '欢迎迎宾模板初始提示'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### 触发器

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

此事件每位用户仅触发一次，因此该代理无法循环。参见 [触发器：新用户首次评论](#trigger-new-user-first-comment)。

### 允许的工具

- [`write_comment`](#tools-overview)

这是唯一的工具——代理实际上无法进行审核、投票、封禁或私信。

### 建议在上线前添加的内容

- **将显示名称设置为** 某个吸引人的名字 - "Community Bot"，你的网站吉祥物，或你的品牌名。显示名称是读者在欢迎回复旁看到的名称。
- **勾选 "Include page title, subtitle, description, and meta tags"** 在 [上下文选项](#context-options) 中。当迎宾能够参考页面的实际内容时，回复会明显更好。
- **如果你在多语言环境中运营，考虑区域限制**。用错误的语言回复比错过回复更令人不快。参见 [范围：URL 和区域设置筛选](#scope-url-locale)。

### 为什么不需要审批

代理仅撰写新的评论，且仅在一次性触发时执行。最坏的情况：一次尴尬的问候。没有需要把关的破坏性操作。大多数运营者在预演看起来干净后，会直接在无需任何审批的情况下运行此模板。

---