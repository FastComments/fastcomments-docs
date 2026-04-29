AI Agents are available on the **Flex** and **Pro** plans. The Creator plan does not include them.

### Ограничења на нивоу плана

Each plan tier sets:

- **Default daily and monthly budget caps.** You can lower these per-agent; raising the per-account cap requires a plan with a higher ceiling. See [Преглед буџета](#budgets-overview).

The exact numbers are shown on the [pricing page](https://fastcomments.com/traffic-pricing) and on your account's billing page. They are also shown inline on the agent edit form so you never have to leave the form to find your cap.

FastComments Pro includes $200/mo worth of AI usage. Flex is billed at the rate of $20 per million tokens for all models (currently either GLM 5.1 or gpt-oss-120B-turbo).

### Подаци о плаћању морају бити важећи

AI Agents only run when the tenant has **важеће податке о плаћању у систему**. If the payment method becomes invalid, all agents are paused and the AI Agents page surfaces a banner directing whoever has the **Billing Admin** role to update billing. Agents resume on their own once billing is restored - no replay or backfill of triggers that fired during the outage.

This is a hard prerequisite: token spend bills against your account, so the platform will not dispatch any LLM call without a working payment method.

### Ко може управљати агентима

The agent admin pages are gated behind the **Customization Admin** dashboard role. **Comment Moderator Admins** can review and decide approvals (see [Процес одобравања](#approval-workflow)) but cannot create or edit agents. **Billing Admins** receive [мејлови са упозорењима о буџету](#budget-alerts) regardless of whether they have agent access.