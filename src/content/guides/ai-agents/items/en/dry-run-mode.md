**Dry Run** is the safety mode every new agent starts in. The agent runs end-to-end except for the part where it touches your community.

### What runs in Dry Run

- Triggers fire normally.
- The agent's prompt, [community guidelines](#community-guidelines), and [context](#context-options) are assembled.
- The LLM is called.
- The model picks tool calls and supplies justifications + confidence scores.
- The run is recorded with a **Dry Run** badge so it is clearly distinguished from live runs.

### What does not run in Dry Run

- No comment is posted, no vote is cast, no comment is pinned/unpinned/locked/unlocked.
- No comment is marked spam, approved, or reviewed.
- No user is banned, warned, or awarded a badge.
- No email is sent.
- No memory is written. (Yes - including memory. Dry-run agents cannot poison the shared memory pool with hypothetical decisions.)
- No webhooks fire for tool actions. (The trigger-level `trigger.succeeded` / `trigger.failed` webhooks do still fire and the payload includes `wasDryRun: true`. See [Webhook Payloads](#webhook-payloads).)

### What it costs

Dry Run runs **the same LLM call** an Enabled run would. Tokens are charged, [budget caps](#budgets-overview) apply, and the runs count against the per-agent and per-tenant daily/monthly limits.

That cost is the price of getting a faithful preview. A "skip the LLM call" mode would not give you any signal about how the agent would behave.

### Reading dry-run results

In [Run History](#run-history), dry-run runs are marked with the **Dry Run** badge in the status column. The actions inside each run look identical to live actions - same tool name, same arguments, same justification and confidence - except none of them happened.

The [Analytics page](#analytics-page) breaks down "dry-run vs live" runs per month so you can see how much of your token spend went to observation.

### Switching out of Dry Run

Edit the agent and change **Status** to **Enabled**. The next trigger runs live.

You can also switch the other way - Enabled back to Dry Run - if the agent starts doing things you do not like. There is no penalty.

### Replays force Dry Run

The [Test Runs (Replays)](#test-runs-replays) feature runs the agent against historical comments **always in dry-run**, regardless of the agent's saved status. Replays cannot take real actions on past comments. This is by design - replay is a preview tool, not a moderation tool.
