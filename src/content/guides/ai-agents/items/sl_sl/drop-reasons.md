When a trigger fires for an agent but does **not** result in an LLM call, the platform records a "drop" with a reason. Drops appear in the [Analytics page](#analytics-page) under "Triggers skipped (this month)".

### The full list of drop reasons

| Razlog | Kaj se je zgodilo |
|---|---|
| `agentDaily` | Dosežen je bil dnevni proračunski limit agenta. |
| `agentMonthly` | Dosežen je bil mesečni proračunski limit agenta. |
| `tenantDaily` | Dosežen je bil dnevni proračunski limit najemnika. |
| `tenantMonthly` | Dosežen je bil mesečni proračunski limit najemnika. |
| `qps` | Dosežena je bila agentova omejitev hitrosti na minuto (drseče okno 60 s). |
| `concurrency` | Maksimalno število sočasnih zagonov agenta je bilo že zasedeno. |

### What's not in this list

A trigger that never reaches the dispatch path is not "dropped" with a reason - it is just not dispatched. That includes:

- Agent is **Disabled**.
- The triggering comment does not match the agent's [URL/locale scope](#scope-url-locale).
- The triggering action was made by the same agent (loop prevention).
- The tenant has invalid billing.
- The agent is not in the tenant's plan.

These are silent skips, not drops. They do not appear in the drops chart on Analytics.

### Reading drops on Analytics

The [Analytics page](#analytics-page) shows:

- **Triggers skipped (this month)** - counts grouped by drop reason.
- **Agents at or near their cap** - per-agent breakdown of which agents are pushing the cap, with a count of dropped triggers in the current period.

### What to do when you see drops

- **`agentDaily` / `agentMonthly`** - the agent's own cap is too tight. Either raise the cap on the edit form or scope the agent down (URL/locale, narrower triggers).
- **`tenantDaily` / `tenantMonthly`** - the account-level cap is too tight. Raise it on tenant billing settings, or distribute spend across fewer agents.
- **`qps`** - traffic is hitting the per-minute rolling-window limit. Often a sign of a viral thread fanning out triggers faster than the agent can run them. The agent's `maxTriggersPerMinute` and `maxConcurrent` fields cap this; raising them increases throughput but also increases burst cost.
- **`concurrency`** - same root cause as `qps` but at the in-flight count. Raise `maxConcurrent` if you need more parallelism.

### Drops vs errors

A drop is "the trigger never ran". An **error** is "the trigger ran but the LLM call or tool dispatch failed". Errors are tracked separately on the [Run History](#run-history) page (status `Error`).

### Drops can also stop replays

The same drop reasons stop in-flight [test runs / replays](#test-runs-replays). The replay stops with an Error status and a message that names which budget was hit (for example, the agent's daily budget).

### Loop prevention is silent on purpose

There is no drop reason for "this trigger came from another agent and was skipped to prevent a loop". Logging it would clutter the analytics for no useful signal - by design, agent fan-out should never result in trigger explosion. If you suspect a loop is being suppressed where it should not be, check [Dnevnike komentarjev](/guide-moderation.html#comment-logs) - the `botId` on bot-authored comments is what the loop check keys on.