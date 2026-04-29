An agent has one of three statuses:

### Disabled

The agent is turned off. No triggers are processed and the agent does not appear in the dispatch path. Its run history, analytics, and memory remain - if you re-enable it later, the historical data is still there.

Use `Disabled` when:
- You want to take an agent out of rotation without losing it.
- An agent is misbehaving and you need to stop it immediately while you investigate.
- You are seasonally rotating agents in and out (e.g. a holiday-only greeter).

### Dry Run - default for new agents

The agent runs end-to-end - it processes triggers, calls the LLM, picks tool calls, computes justifications and confidence - but **no real action is taken**. Each run is recorded with the **Dry Run** badge in [Run History](#run-history).

Use `Dry Run` when:
- A new agent is just out of the box. Every starter template lands in dry-run.
- You have edited the prompt or changed the trigger set and want to see how the change plays out before committing.
- You are running a [test run / replay](#test-runs-replays) (replays force dry-run regardless of agent status).

The platform charges tokens for dry-run runs - the LLM call still happens, only the side-effects are skipped. Budget caps apply to dry-run too. See [Budgets Overview](#budgets-overview).

### Enabled

The agent takes real actions. Tool calls execute - or get queued for [approval](#approval-workflow) if the action is gated.

Use `Enabled` after dry-run output looks correct.

### Switching status

You can flip between any two statuses on the edit form. Switching from Dry Run to Enabled does not retroactively re-execute the dry-run actions - those stay as dry-run history. New triggers from that moment forward run live.

Switching from Enabled to Disabled mid-run does **not** abort an in-flight run. The currently-executing trigger finishes (with whatever it has already started); the next trigger is dropped because the agent is now Disabled.

### Status during billing problems

If your tenant's billing becomes invalid, all agents are effectively paused regardless of saved status - triggers are dropped with `BILLING_INVALID` until billing is restored. The saved status field is not changed; the dispatcher just refuses to run. See [Plans and Eligibility](#plans-and-eligibility).
