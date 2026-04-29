Τα email ειδοποίησης προϋπολογισμού ενεργοποιούνται όταν η δαπάνη ενός agent υπερβεί ένα ρυθμιζόμενο ποσοστό του ορίου του. Απευθύνονται στα άτομα που είναι υπεύθυνα για το τιμολόγιο.

### How alerts work

Every agent has an **Alert thresholds** field on the edit form. By default it is `80%` and `100%`. You can tick or untick individual thresholds, and you can add other percentages.

When the agent's spend in a given scope (daily or monthly) crosses a threshold for the first time in that period, the platform sends one email per recipient. Crossing the threshold again later in the same period (e.g., spend dipped below 80% and came back over) does **not** re-send.

This is per-period: a new daily reset starts the threshold-crossing logic over for that day.

### Tenant-scope alerts

The tenant (account) has its own daily and monthly caps. Tenant-scope alerts fire at fixed thresholds (`80%` and `100%`). These are not configurable per-agent because they apply to the whole tenant.

### Recipients

Budget alerts are sent to:

- Every user marked **Super admin** on the tenant.
- Every user marked **Billing Admin** on the tenant.

That includes the union of the two roles - a user with both roles gets one email.

### Why both roles

Super admins are typically the operators who need to know an agent is hitting its cap. Billing admins own the invoice and need to know about cost spikes regardless of whether they manage agents day to day. To actually edit the agent (raise the cap, pause it), the recipient also needs the **Customization Admin** role - which gates the agent edit page.

### Per-user opt-out

Recipients who have opted out of admin notifications on their profile are skipped. This is the same opt-out switch that controls other admin notifications.

If **all** recipients are opted-out, the alert is logged (warning level) and no email is sent.

### Email content

The email contains:

- The **agent display name** and internal name.
- The **scope** that crossed (e.g., "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- The **threshold percentage** crossed.
- **Usage** in the tenant's currency.
- **Cap** in the tenant's currency.
- A **one-click signed login link** that takes the recipient straight to:
  - The agent edit page, for agent-scope alerts.
  - The AI Agents list page, for tenant-scope alerts.

The link is pre-authenticated, so the recipient is one click away from raising the cap or disabling the agent.

### How thresholds fire

The platform tracks which thresholds have already fired this period, separately for the agent and the tenant. So:

- Crossing 80% then 100% in the same period fires both, in order.
- Going straight from 0% to 100% in one big jump fires the **highest** crossed threshold (100%), not 80%, so the most-severe alert is the one delivered.

### When you stop getting alerts

If the agent's spend never reaches the next threshold this period, you do not get further emails this period. The next daily reset (or monthly reset) clears the tracking.

### Disabling alerts

Untick the threshold you do not want. If you do not want any alerts on a specific agent, untick all percentages. Tenant-scope alerts cannot be disabled per-agent (they are tenant-wide).

### See also

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - what happens when the cap is fully reached.
- [Cost Model](#cost-model) - what's being measured.