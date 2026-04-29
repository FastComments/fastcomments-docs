**模板 ID:** `tos_enforcer`

The Moderator template is the recommended starting point if your goal is reducing manual moderation load. It reviews new and flagged comments and applies your community rules.

### Built-in initial prompt

[inline-code-attrs-start title = '版主模板初始提示'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

You will almost always want to **augment this prompt** with concrete examples of what your site does and does not allow. The platform's own escalation policy (warn before ban, search memory before banning) is already baked into the system prompt the agent receives, so you do not need to repeat it.

### Triggers

- **新评论发布** (`COMMENT_ADD`) - 代理会查看每条新评论。
- **评论达到标记阈值** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - 当其他用户标记的评论达到阈值时，代理会重新评估该评论。

### Allowed tools

- [`mark_comment_approved`](#tools-overview) - 对于预审核租户，代理可以释放干净的评论并隐藏其余内容。
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

它不能发布评论、投票、置顶、锁定、授予徽章或发送电子邮件 —— 该提示刻意设置为较窄的权限。

### Recommended additions before going live

- **设置[社区准则](#community-guidelines)。** 几句书面的政策说明就足够；代理会在每次运行时应用它。
- **将 `ban_user` 操作置于[批准](#approval-workflow)之后。** 这在欧盟地区默认开启（参见[欧盟 DSA 第 17 条合规](#eu-dsa-compliance)），并且在所有地区都推荐启用。
- **如果您拥有低流量但高风险的内容，考虑也将 `mark_comment_spam` 操作置于批准流程后。**
- **如果您运行预审核（pre-moderation），将 `mark_comment_approved` 置于批准流程后。** 批准不良评论会将其展示给读者；在代理通过干运行赢得信任之前请将其置于审核保护下。
- **在[上下文选项](#context-options)中勾选“包含评论者的信任因子、账户年龄、封禁历史和近期评论”。** 当模型能够看到某人是长期良好信誉的用户时，它会减少强烈的警告。

### Recommended dry-run window

在将其切换为“启用”之前，至少对真实流量以[干运行](#dry-run-mode)模式运行该模板一周。使用[Test Runs (Replays)](#test-runs-replays)也可以预览过去 30 天的数据。