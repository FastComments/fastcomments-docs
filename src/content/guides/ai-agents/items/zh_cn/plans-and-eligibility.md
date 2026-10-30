AI 代理在 **Flex** 和 **Pro** 计划中可用。Creator 计划不包括它们。

### Plan-level limits

每个计划层级设置：

- **默认每日和每月预算上限。** 您可以为每个代理降低这些上限；提升每个账户的上限需要更高上限的计划。参见 [Budgets Overview](#budgets-overview)。

确切的数字显示在[pricing page](https://fastcomments.com/traffic-pricing)和您账户的账单页面上。它们也会在代理编辑表单中内联显示，这样您无需离开表单即可查看上限。

FastComments Pro 包含价值 $200/mo 的 AI 使用量。Flex 按每百万 token $20 的费率计费，适用于所有模型（目前为 GLM 5.2 或 gpt-oss-120B）。

### Billing must be valid

AI 代理仅在租户拥有 **valid billing on file** 时运行。如果付款方式失效，所有代理将被暂停，AI Agents 页面会显示横幅，指示拥有 **Billing Admin** 角色的人员更新账单。账单恢复后，代理会自行恢复运行——不会重新执行或补填在停机期间触发的触发器。

这是一个严格的前提条件：token spend 计入您的账户，因此平台在没有有效付款方式的情况下不会发起任何 LLM 调用。

### Who can manage agents

代理管理页面受 **Customization Admin** 仪表板角色的限制。**Comment Moderator Admins** 可以审查并决定批准（参见 [Approval Workflow](#approval-workflow)），但不能创建或编辑代理。**Billing Admins** 无论是否拥有代理访问权限，都会收到 [budget alert emails](#budget-alerts)。