Every agent has spend caps. The platform stops dispatching the agent when a cap is reached and resumes once the period rolls over.

### Two scopes, two periods

There are four caps in total - two scopes (per-agent, per-tenant) crossed with two periods (daily, monthly).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

A trigger only dispatches if **all four caps** allow it. The first cap to be exhausted is the one that drops the trigger.

### Currency

Per-agent budgets are entered in your account currency.

### What happens when a cap is reached

- The trigger is recorded as **dropped** with a [drop reason](#drop-reasons) like `agentDaily` or `tenantMonthly`.
- The dropped count appears on the [Analytics page](#analytics-page) under "Triggers skipped (this month)".
- No LLM call is made; no tokens are spent on the dropped trigger itself.
- The agent's status is unchanged - it just can't dispatch until the period rolls over.

### Period roll-over

- **Daily** caps reset at midnight UTC.
- **Monthly** caps reset at the start of each calendar month, UTC.

There is no rollover of unused budget into the next period.

### Hard cap vs soft warnings

Caps are **hard**. There is no "exceed by 10% with warning" mode. When the cap is reached, dispatch stops.

The "soft" part is the [Budget Alerts](#budget-alerts) emails - you get an email at configurable thresholds (default 80% and 100%) so you can raise the cap before traffic starts dropping.

### Where to read current usage

- [Analytics page](#analytics-page) - per-agent and tenant-wide budget usage with cap markers.
- The agent edit form's **Stats** section.
- The list view (count of pending approvals and recent runs is on the agent card).

### Picking a budget

A few rules of thumb:

- **A new agent** - determine budget. Watch [Run History](#run-history) for a week. Adjust based on observed cost per run × expected trigger volume.
- **A high-volume agent** (e.g., new-comment trigger on a busy site) - the daily cap is what catches a runaway loop. Pick a daily cap that is 2-3x your expected daily spend so a normal busy day fits comfortably under it.
- **A summarizer or context-heavy agent** - per-run cost is high. Set a tighter daily cap to prevent a bad day from blowing the monthly.

### Budget bypass for replays

[Test runs / replays](#test-runs-replays) are subject to their **own** hard cap (set on the replay form, separate from the agent's daily/monthly caps), AND to the agent and tenant caps. Whichever is hit first stops the replay.

### See also

- [Budget Alerts](#budget-alerts) for the email notifications.
- [Cost Model](#cost-model) for how the platform converts tokens to dollars.
- [Drop Reasons](#drop-reasons) for the full list of reasons a trigger does not run.
