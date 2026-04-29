AI Agents 在 **Flex** 和 **Pro** 計劃可用。Creator 計劃不包含它們。

### 計劃層級限制

Each plan tier sets:

- **Default daily and monthly budget caps.** 您可以為每個代理降低這些上限；要提高每個帳戶的上限則需要更高上限的計劃。請參閱 [預算總覽](#budgets-overview)。

The exact numbers are shown on the [定價頁面](https://fastcomments.com/traffic-pricing) and on your account's billing page. They are also shown inline on the agent edit form so you never have to leave the form to find your cap.

FastComments Pro includes $200/mo worth of AI usage. Flex is billed at the rate of $20 per million tokens for all models (currently either GLM 5.1 or gpt-oss-120B-turbo).

### 帳單必須有效

AI Agents only run when the tenant has **valid billing on file**. If the payment method becomes invalid, all agents are paused and the AI Agents page surfaces a banner directing whoever has the **Billing Admin** role to update billing. Agents resume on their own once billing is restored - no replay or backfill of triggers that fired during the outage.

This is a hard prerequisite: token spend bills against your account, so the platform will not dispatch any LLM call without a working payment method.

### 誰可以管理 agents

The agent admin pages are gated behind the **Customization Admin** dashboard role. **Comment Moderator Admins** can review and decide approvals (see [審核工作流程](#approval-workflow)) but cannot create or edit agents. **Billing Admins** receive [預算警示電子郵件](#budget-alerts) regardless of whether they have agent access.