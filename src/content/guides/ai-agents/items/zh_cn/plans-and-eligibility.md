AI Agents are available on the **Flex** and **Pro** plans. The Creator plan does not include them.

### 计划级别限制

Each plan tier sets:

- **默认的每日和每月预算上限。** 您可以按每个 agent 降低这些上限；要提高每个账户的上限则需要更高级别的计划。参见 [预算概览](#budgets-overview).

The exact numbers are shown on the [定价页面](https://fastcomments.com/traffic-pricing) and on your account's billing page. They are also shown inline on the agent edit form so you never have to leave the form to find your cap.

FastComments Pro includes $200/mo worth of AI usage. Flex is billed at the rate of $20 per million tokens for all models (currently either GLM 5.1 or gpt-oss-120B-turbo).

### 计费必须有效

AI Agents only run when the tenant has **valid billing on file**. If the payment method becomes invalid, all agents are paused and the AI Agents page surfaces a banner directing whoever has the **Billing Admin** role to update billing. Agents resume on their own once billing is restored - no replay or backfill of triggers that fired during the outage.

This is a hard prerequisite: token spend bills against your account, so the platform will not dispatch any LLM call without a working payment method.

### 谁可以管理 agents

The agent admin pages are gated behind the **Customization Admin** dashboard role. **Comment Moderator Admins** can review and decide approvals (参见 [审批工作流程](#approval-workflow)) but cannot create or edit agents. **Billing Admins** receive [预算提醒邮件](#budget-alerts) regardless of whether they have agent access.