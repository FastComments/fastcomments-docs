Analytics is the cross-agent dashboard. Reachable from the AI Agents page via the **Analytics** tab (tenant-wide) or per-agent via the **Analytics** button on each agent's row.

### Filter

A dropdown at the top - **All agents** or a specific agent. The rest of the page rescopes accordingly.

### Budget usage

Four progress bars showing current period spend against cap:

- **Agent today** (when filtered to a specific agent) - daily agent cap.
- **Agent this month** - monthly agent cap.
- **Account today** - tenant daily cap.
- **Account this month** - tenant monthly cap.

When a cap is unset, the bar reads "(no cap set)" and shows the raw spend.

### Daily cost (last 30 days)

A table of per-day cost in your tenant's currency for the selected scope. Useful for spotting:

- **Sudden cost spikes** - usually from a runaway loop or a viral comment fanning out triggers.
- **Cost drift** - gradually increasing daily cost as your community grows.

### Actions taken

A breakdown of action types over the current month - "Wrote a comment: 47", "Marked a comment as spam: 12", and so on. Useful for sanity-checking that the agent is doing what you expected.

### Triggers skipped (this month)

Counts grouped by [drop reason](#drop-reasons):

- Over agent daily / agent monthly / account daily / account monthly.
- Rate-limited.
- Concurrency saturated.

If you see drops here, your agent is hitting a budget or rate limit and is missing triggers it would otherwise have run on. See [Drop Reasons](#drop-reasons).

### Dry-run vs live (this month)

- **Enabled runs** - count of runs that took real actions this month.
- **Dry runs** - count of runs in dry-run mode this month.

A useful tuning signal: a brand-new agent that has not yet been promoted to Enabled will show only dry runs. An agent in Enabled with all-zero counts in this section is sitting idle - either it's not being triggered, or it's being scoped out, or its triggers are not configured correctly.

### Top agents by monthly cost

When filter is **All agents**, the page lists agents ranked by month-to-date cost. Spotting your most expensive agent is the first step in cost optimization - usually the answer is "tighten its [context options](#context-options)" or "lower its [budget cap](#budgets-overview)".

### Agents at or near their cap

Per-agent breakdown of agents whose spend is at or near their per-agent caps in the current period:

- **near cap** - over a configurable percentage of the cap.
- **over cap** - actually capped, with `{count} dropped` triggers in that period.

Click into the agent from this table to raise the cap, narrow scope, or pause it.

### Account summary

When filter is **All agents**:

- **Triggers today** - count.
- **Triggers this month** - count.
- For each: a `dropped` suffix showing how many were skipped.

### Currency

All monetary values are shown in your tenant's currency.

### What this page does not do

- It does not show **per-action cost breakdowns** - those are on [Run Detail View](#run-detail-view).
- It does not show **transcripts** or **LLM responses**.
- It does not let you take action on agents - editing, pausing, deleting are all done from the agent list / edit page.
