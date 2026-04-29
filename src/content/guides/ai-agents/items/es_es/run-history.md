Run History es el registro por agente de cada trigger que se ejecutó. Accesible desde la página de lista de agentes mediante el botón **Runs**, o directamente en `/auth/my-account/ai-agents/{agentId}/runs`.

### What's on the page

A paginated table with one row per run:

| Column | Meaning |
|---|---|
| Date | When the trigger fired (or when the deferred trigger ran). |
| Status | **Started**, **Success**, or **Error**. **Dry Run** badge is shown alongside if the run was in dry-run mode. |
| Cost | Per-run cost in your tenant's currency. Empty for in-flight (Started) runs. |
| Actions | The number of tool calls in the run. |
| Details | A **View** button that opens [Run Detail View](#run-detail-view). |

### Status meanings

- **Started** - the run is in progress, or it died before completing. A run stuck in "Started" for an unusually long time usually represents an LLM-call timeout.
- **Error** - the run completed but failed somewhere - LLM call returned an error, a tool dispatch failed, etc. The detail view contains the specific error.
- **Success** - the run completed without error. The agent may have taken zero, one, or many actions.

### Empty state

When an agent has no runs, the page shows: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

That last bit is intentional - the [test run flow](#test-runs-replays) is the recommended way to populate Run History on a fresh agent.

### What's not on the run history page

- **Live triggers that never dispatched** - a trigger dropped because of budget, scope, or rate limiting does not appear on this page. Those show up in the [Analytics page](#analytics-page) under "Triggers skipped".
- **Approvals** - pending approvals for actions taken in this run live in the [approvals inbox](#approval-workflow). The action shows up on the run detail view as **Pending approval**.

### Retention

Individual run records are retained for 90 days, after which the run is gone from history. Cost and trigger counts continue to roll up in long-term analytics summaries, so the [Analytics page](#analytics-page) still shows historical totals beyond that window.

### Replays

Replay-produced runs are excluded from the live-runs view by default. The [Test Runs (Replays)](#test-runs-replays) page is where you see those.

### Filtering across agents

The runs table is per-agent. There is no cross-agent runs view - the [Analytics page](#analytics-page) is the cross-agent summary. If you need to inspect runs across multiple agents, the [Webhooks](#webhooks-overview) `trigger.succeeded` and `trigger.failed` events are what you would forward to your own system.

---