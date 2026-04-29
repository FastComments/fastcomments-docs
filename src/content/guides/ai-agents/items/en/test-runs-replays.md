A **test run** (also called a **replay**) runs the agent against a window of historical comments **without taking real actions**. It is the fastest way to preview agent behavior before going live.

Reachable from the agents list page via the **Test run** button on each agent's row.

### What it does

The platform:

1. Selects a sample of historical comments matching the agent's scope, in the window you choose.
2. For each comment, runs the agent end-to-end as if the comment had just been posted - same context, same LLM call, same tool selection, same justifications and confidence scores.
3. Records every run as a dry-run, tagged so it stays grouped with the replay it came from and excluded from live-run views.
4. **Compares** the agent's verdict to what actually happened to the comment - was it later approved, marked spam, deleted, blocked by spam engine, etc.

The result is a per-comment diff: "Replay agent would mark this as spam, but the comment is currently approved and clean."

### Configuration

The test-run page has a single input:

- **Days of historical comments to evaluate** - a numeric `days` field between 1 and 90. Older comments are not eligible.

Sample size and hard cap are **not exposed in the UI** - both are server-side defaults applied per plan. The page shows informational fields:

- **Matching comments in window** - how many comments would be considered.
- **Up to N comments from this window will be processed** - the effective sample size given the server-side cap.
- **Estimated cost** - in your tenant's currency.

### Rate limit

Each user is limited to **10 test runs per 24 hours** (rate-limited via key `replay-create:${requestedBy}`). The button shows a tooltip when you have hit the limit ("You've reached 10 test runs in the last 24 hours.").

### Concurrency

Only one replay can be active per agent at a time. Starting a second replay while one is in flight redirects you to the in-flight one.

### Reading results

When the replay finishes, the result page shows tabs:

- **Deltas** (default-active) - replay agent's verdict differs from reality. (Most interesting - "the agent would have spam-marked this comment, but the comment was approved and is fine".)
- **Matches** - replay agent's verdict matches what actually happened. (Reassuring - the agent agrees with reality.)
- **No action** - replay agent decided to do nothing. (Sometimes the right answer; sometimes the agent missed something.)
- **All** - every result regardless of classification.

For each comment in any tab:

- **Prior outcome** - the classification of what actually happened: **POSITIVE**, **NEGATIVE**, or **INDETERMINATE**, with **Evidence** ("Comment marked deleted at {date}", "Engine: bayes", and so on).
- **Replay agent would** - the action the agent picked.
- **Why** - the justification.
- **Confidence** - displayed as a percentage.

### Why replays force dry-run

A replay against a comment that was deleted four months ago should not retroactively delete it - it is already deleted. A replay against a comment that the agent now wants to approve should not change the comment's current state. Replay is a preview tool. Forcing dry-run is what makes it safe to run a replay against any window of history.

### Reproducibility

Replays freeze the agent's config at the moment the replay was started. Subsequent edits to the agent do not change the replay's results - the result page remains stable as a record of what *that* version of the agent would have done.

### When budgets stop a replay

Replays are subject to:

- Their own **hard cap** (set on the replay form).
- The agent's daily and monthly **budget caps**.
- The tenant's daily and monthly **budget caps**.

The first one hit aborts the replay with a specific error code. Any per-comment results produced before the abort are preserved in [Run History](#run-history).

### How replays run

Replays run in the background, not synchronously. After you click "Start test run", the replay is queued and a worker picks it up. A long replay can span several minutes. The result page polls and shows progress (processed count, spend so far) as it goes.

If a worker dies mid-replay, the platform automatically requeues the replay so it resumes on the next pass. A brief blip never orphans a replay.

### What replay does not do

- **Does not honor [trigger delays](#trigger-deferred-delay).** Replays run immediately, not 30 minutes later.
- **Does not write to memory.** Replay agents do not save memory notes, even if their logic would normally.
- **Does not fire webhooks.** Replay-produced triggers do not generate `trigger.succeeded` / `trigger.failed` webhook events.
- **Does not exclude already-replayed comments.** Running a second replay against the same window covers the same comments.

### See also

- [Refining Prompts](#refining-prompts) - the iterative-edit workflow that pairs well with replays.
- [Dry-Run Mode](#dry-run-mode) - the same idea, against live traffic.
