AI Agents 在 **Flex** 和 **Pro** 套餐中可用。Creator 套餐不包含此功能。

### 套餐级限制

每个套餐级别设置了：

- **默认的每日和每月预算上限。** 您可以为每个 Agent 单独调低这些上限；提高账户级上限则需要更高上限的套餐。请参阅 [预算概览](#budgets-overview)。

具体数字显示在 [定价页面](https://fastcomments.com/traffic-pricing) 和您账户的账单页面上。它们也会直接显示在 Agent 编辑表单中，因此您无需离开表单即可查看您的上限。

FastComments Pro 包含每月 $200 的 AI 使用额度。Flex 按所有模型（目前为 GLM 5.2 或 gpt-oss-120B）每百万 token $20 的费率计费。

### 账单必须有效

只有当租户存有**有效账单信息**时，AI Agents 才会运行。如果支付方式失效，所有 Agent 将被暂停，并且 AI Agents 页面会显示一个横幅，指引拥有 **Billing Admin** 角色的用户更新账单信息。账单恢复后，Agent 会自动恢复运行——不会重放或回填中断期间触发的触发器。

这是一个硬性前提条件：token 消耗会计入您的账户账单，因此平台在没有有效支付方式的情况下不会分发任何 LLM 调用。

### 谁可以管理 Agent

Agent 管理页面受 **Customization Admin** 仪表板角色限制。**Comment Moderator Admins** 可以审核并决定批准（请参阅 [审批工作流](#approval-workflow)），但不能创建或编辑 Agent。**Billing Admins** 无论是否拥有 Agent 访问权限，都会收到 [预算提醒邮件](#budget-alerts)。