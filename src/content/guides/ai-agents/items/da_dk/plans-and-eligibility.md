AI Agents are available on the **Flex** and **Pro** plans. The Creator plan does not include them.

### Plan-level limits

Each plan tier sets:

- **Default daily and monthly budget caps.** You can lower these per-agent; raising the per-account cap requires a plan with a higher ceiling. See [Budgetoversigt](#budgets-overview).

The exact numbers are shown on the [pricing page](https://fastcomments.com/traffic-pricing) and on your account's billing page. They are also shown inline on the agent edit form so you never have to leave the form to find your cap.

FastComments Pro includes $200/mo worth of AI usage. Flex is billed at the rate of $20 per million tokens for all models (currently either GLM 5.1 or gpt-oss-120B-turbo).

### Billing must be valid

AI Agents only run when the tenant has **valid billing on file**. If the payment method becomes invalid, all agents are paused and the AI Agents page surfaces a banner directing whoever has the **Billing Admin** role to update billing. Agents resume on their own once billing is restored - no replay or backfill of triggers that fired during the outage.

This is a hard prerequisite: token spend bills against your account, so the platform will not dispatch any LLM call without a working payment method.

### Who can manage agents

The agent admin pages are gated behind the **Customization Admin** dashboard role. **Comment Moderator Admins** can review and decide approvals (see [Godkendelsesworkflow](#approval-workflow)) but cannot create or edit agents. **Billing Admins** receive [budgetadvarsels-e-mails](#budget-alerts) regardless of whether they have agent access.