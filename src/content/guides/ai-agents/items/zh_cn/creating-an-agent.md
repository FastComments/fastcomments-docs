从 [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) 页面，您可以通过两种方式创建代理：

- **From a template.** 点击 **Browse templates** 并选择四个内置入门代理之一。表单将预填，代理的状态为 **Dry Run**。参见 [Starter Templates](#starter-templates)。
- **From scratch.** 点击 **Create new agent**。表单为空。

无论哪种方式，保存后编辑的都是同一个编辑表单。本页自上而下介绍该表单。

### Basics

- **Internal name.** 仅在管理仪表板中使用的简短标识符（运行历史、分析、审计日志）。小写并使用下划线效果很好： `moderator`, `welcome_greeter`。如果模板的内部名称已被占用，表单会自动添加后缀（`tos_enforcer_2` 等）。
- **Display name.** 当代理发布评论时公开显示。这是读者看到的名称。
- **Status.** Disabled、Dry Run 或 Enabled。新代理默认始终为 Dry Run。参见 [Status States](#status-states)。

### Model

选择 LLM。参见 [Choosing a Model](#choosing-a-model)。

### Budget

可选的每日和每月上限（以您的账户货币计），以及一个 **Alert thresholds** 复选清单（默认 80% 和 100%）。参见 [Budgets Overview](#budgets-overview) 和 [Budget Alerts](#budget-alerts)。

### Personality

**Initial prompt** 是定义语气、角色和决策规则的系统提示。纯文本，不可使用模板语法。参见 [Personality and the Initial Prompt](#personality-prompt)。

### Context

Context 字段集包含三个复选框、一个指南文本区域和范围输入：

- 包含父评论和同一线程中的先前回复。
- 包含评论者的信任因子、账户年龄、禁令历史和最近的评论。
- 包含页面标题、副标题、描述和元标签。
- 一个可选的 **Community guidelines** 文本块，会被预先添加到每个提示中。
- **Restrict to specific pages** - URL 模式允许列表（每行一个）。为空表示适用于整个租户。
- **Restrict to specific locales** - 通过双列表选择器设置区域允许列表。为空表示适用于所有区域。

更多的上下文能带来更好的决策，但会提高每次运行的 token 成本。参见 [Context Options](#context-options)、[Community Guidelines](#community-guidelines) 和 [Scope: URL and Locale Filters](#scope-url-locale)。

### Triggers

从列表中至少选择一个事件。对于 vote-threshold 和 flag-threshold 触发器，您还必须设置阈值。可选的 **Delay before running** 字段可在触发器触发后延迟执行（对投票仍在结算的 flag 阈值情况很有用）。参见 [Trigger Events Overview](#triggers-overview) 和 [Deferred Triggers](#trigger-deferred-delay)。

### Allowed tool calls

勾选 **Allow any tool calls** 可公开完整的工具面板。否则请勾选代理被允许使用的具体工具——不被允许的工具将在模型的工具面板中被剔除并在调度时被拒绝。**Ban options** 子部分将破坏性禁令变体（`delete-all-comments`, `ban-by-IP`）放在显式选择之后。参见 [Allowed Tool Calls Overview](#tools-overview) 和 [Tool: ban_user](#tool-ban-user)。

### Approvals

勾选需要在人类批准后代理才能执行的操作。审批仅适用于代理被允许调用的工具；被拒绝的工具将被直接拒绝。在欧盟地区，**ban_user** 根据《数字服务法》第17条被强制启用。参见 [Approval Workflow](#approval-workflow) 和 [EU DSA Article 17 Compliance](#eu-dsa-compliance)。

### Approval notifications

如果启用了审批，请选择谁会收到电子邮件：

- **All admins and moderators** - 帐户所有者、超级管理员和评论版主管理员。
- **Specific users** - 从双列表选择器中手动挑选。

每位审阅者的单独投递频率（即时、按小时摘要、按日摘要）在其个人资料中设置。参见 [Approval Notifications](#approval-notifications)。

### Stats

只读。总运行次数、上次运行时间戳，以及代理撰写的最新评论的 ID（如果有的话）。

### Save

点击 **Save agent**。页面将重定向回代理列表。新代理在试运行状态下立即有资格接收触发器。

### Editing later

代理列表页面的每一行都提供每个代理的操作：**Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, 和 **Delete**。编辑代理不会追溯影响已记录的运行——历史会被保留。重放快照也会在启动重放时冻结代理的配置，因此已保存的重放结果即使在您编辑提示后仍能保持可复现。