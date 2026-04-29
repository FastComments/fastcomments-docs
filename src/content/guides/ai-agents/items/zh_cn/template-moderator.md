---
**模板 ID：** `tos_enforcer`

Moderator 模板是如果您的目标是减少人工审核工作量时推荐的起点。它会审查新的和被标记的评论并应用您的社区规则。

您几乎总是需要用有关您网站允许与不允许内容的具体示例来**扩展内置提示**。平台自身的升级策略（在封禁前先警告，在封禁前先检查记忆）已经写入代理接收的系统提示中，因此您无需重复它。

### 触发器

- **New comment posted** (`COMMENT_ADD`) - 代理会查看每一条新评论。
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, 默认阈值：3) - 代理会重新评估被其他用户标记的评论。

### 允许的工具

- [`mark_comment_approved`](#tools-overview) - 对于采用预审的租户很有用，代理会发布合规的评论并隐藏其余评论。
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

它不能发布评论、投票、置顶、锁定、授予徽章或发送电子邮件——提示故意被限定得很窄。

### 上线前推荐补充

- **设置[社区准则](#community-guidelines)。** 几句书面政策就足够；代理在每次运行时都会应用它。
- **将 `ban_user` 放在[审批](#approval-workflow)之后。** 在欧盟地区默认启用（参见 [欧盟 DSA 第17条合规](#eu-dsa-compliance)），在所有地区都推荐这样做。
- **如果您的内容量低但影响重大，考虑也将 `mark_comment_spam` 放在审批之后。**
- **如果您采用预审，请将 `mark_comment_approved` 放在审批之后。** 批准错误的评论会将其呈现给读者；在代理通过干运行赢得信任之前，应对其进行审批。
- 在[上下文选项](#context-options)中勾选“包含评论者的信任因子、账户时长、封禁历史和近期评论”。当模型看到某人是长期诚信用户时，它会减少过度警告。

### 推荐的干运行周期

在将此模板切换为启用之前，至少在真实流量下以 [干运行](#dry-run-mode) 模式运行一周。使用 [测试运行（重放）](#test-runs-replays) 还可以对过去 30 天进行预览。

---