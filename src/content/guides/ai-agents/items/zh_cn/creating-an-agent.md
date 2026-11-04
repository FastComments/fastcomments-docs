在 [AI Agents 页面](https://fastcomments.com/auth/my-account/ai-agents)中，您可以通过两种方式创建代理：

- **从模板创建。** 点击 **Browse templates**，从四个内置的入门代理中选择一个。表单会自动预填充，代理的状态为 **Dry Run**。参见 [Starter Templates](#starter-templates)。
- **从零创建。** 点击 **Create new agent**。表单为空白。

无论哪种方式，之后保存和编辑的都是同一个编辑表单。本页面将自上而下地介绍该表单。

### 基本信息（Basics）

- **内部名称（Internal name）。** 仅在管理后台（运行历史、分析、审计日志）中使用的简短标识符。使用小写字母加下划线效果较好：`moderator`、`welcome_greeter`。如果模板的内部名称已被占用，表单会自动添加后缀（如 `tos_enforcer_2` 等）。
- **显示名称（Display name）。** 代理发布评论时公开展示的名称，即您的读者所看到的名称。
- **状态（Status）。** Disabled、Dry Run 或 Enabled。新代理默认始终为 Dry Run。参见 [Status States](#status-states)。

### 模型（Model）

选择 LLM。参见 [Choosing a Model](#choosing-a-model)。

### 预算（Budget）

可选的每日和每月上限（以您账户的货币计），以及一个 **Alert thresholds**（警报阈值）清单（默认为 80% 和 100%）。参见 [Budgets Overview](#budgets-overview) 和 [Budget Alerts](#budget-alerts)。

### 个性（Personality）

**Initial prompt**（初始提示词）是定义语气、角色和决策规则的系统提示词。纯文本，不含模板语法。参见 [Personality and the Initial Prompt](#personality-prompt)。

### 上下文（Context）

Context 字段集包含三个复选框、一个准则文本区域以及范围输入项：

- 包含同一会话中的父评论和之前的回复。
- 包含评论者的信任因子、账户年龄、封禁历史和近期评论。
- 包含页面标题、副标题、描述和 meta 标签。
- 一个可选的 **Community guidelines**（社区准则）文本块，会被添加到每个提示词的前面。
- **Restrict to specific pages**（限制到特定页面）— URL 模式白名单（每行一个）。留空表示对整个租户生效。
- **Restrict to specific locales**（限制到特定语言区域）— 通过双列表选择器设置的语言区域白名单。留空表示所有语言区域。

更多的上下文能产生更好的决策，但会增加每次运行的 token 成本。参见 [Context Options](#context-options)、[Community Guidelines](#community-guidelines) 和 [Scope: URL and Locale Filters](#scope-url-locale)。

### 触发器（Triggers）

从列表中选择至少一个事件。对于投票阈值和举报阈值触发器，您还必须设置阈值。可选的 **Delay before running**（运行前延迟）字段会在触发器触发后延迟执行（对于投票仍在稳定中的举报阈值很有用）。参见 [Trigger Events Overview](#triggers-overview) 和 [Deferred Triggers](#trigger-deferred-delay)。

### 允许的工具调用（Allowed tool calls）

勾选 **Allow any tool calls**（允许任何工具调用）以开放完整的工具面板。否则，请勾选允许代理使用的特定工具 — 未被允许的工具会从模型的工具面板中移除，并在分发时被拒绝。**Ban options**（封禁选项）子部分将破坏性封禁变体（删除所有评论、按 IP 封禁）置于显式选择开启之后。参见 [Allowed Tool Calls Overview](#tools-overview) 和 [Tool: ban_user](#tool-ban-user)。

### 审批（Approvals）

勾选在代理执行之前必须由人工审批的操作。审批仅适用于代理被允许调用的工具；未被允许的工具会被直接拒绝。在欧盟地区，根据《数字服务法》第 17 条，**ban_user** 被强制锁定为开启。参见 [Approval Workflow](#approval-workflow) 和 [EU DSA Article 17 Compliance](#eu-dsa-compliance)。

### 审批通知（Approval notifications）

如果启用了审批，请选择通过电子邮件通知哪些人：

- **所有管理员和版主** — 账户所有者、超级管理员和评论版主管理员。
- **特定用户** — 从双列表选择器中手动挑选。

每位审核者个人的接收频率（即时、每小时摘要、每日摘要）在其个人资料中设置。参见 [Approval Notifications](#approval-notifications)。

### 统计（Stats）

只读。总运行次数、上次运行时间戳，以及代理最近撰写的一条评论的 ID（如有）。

### 保存（Save）

点击 **Save agent**。页面会重定向回代理列表。新代理立即有资格在 dry-run 模式下接收触发器。

### 后续编辑

代理列表页面的每一行都提供针对单个代理的操作：**Edit**、**Clone**、**Runs**、**Replays**、**Test run**、**Analytics**、**Approvals** 和 **Delete**。编辑代理不会追溯影响已记录的运行 — 历史记录会被保留。重放快照还会在重放开始时冻结代理的配置，因此即使您之后编辑了提示词，已保存重放的结果仍可复现。