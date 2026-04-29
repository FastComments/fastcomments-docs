A **trigger** is an event that wakes an agent up. Each agent can have one or more triggers defined.

### The full list

| Trigger | When it fires |
|---|---|
| [Comment Added](#trigger-comment-add) | A new comment is posted. |
| [Comment Edited](#trigger-comment-edit) | A comment is edited. The previous text is included in the agent's context. |
| [Comment Deleted](#trigger-comment-delete) | A comment is deleted. |
| [Comment Pinned](#trigger-comment-pin) | A comment is pinned (by anyone, including a moderator or another agent). |
| [Comment Unpinned](#trigger-comment-unpin) | A comment is unpinned. |
| [Comment Locked](#trigger-comment-lock) | A comment is locked (no further replies allowed). |
| [Comment Unlocked](#trigger-comment-unlock) | A comment is unlocked. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | A comment's net votes reach the configured threshold. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | A comment's flag count reaches exactly the configured threshold. |
| [User Posts First Comment](#trigger-new-user-first-comment) | A user posts their first comment on this site. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | A comment is auto-flagged as spam by the spam engine. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | A moderator marks a comment as reviewed. |
| [Moderator Approves Comment](#trigger-moderator-approved) | A moderator approves a comment. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | A moderator marks a comment as spam. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | A moderator awards a badge to a user. |

### Multiple triggers per agent

An agent can subscribe to any combination of triggers - the [Moderator template](#template-moderator) subscribes to both `COMMENT_ADD` and `COMMENT_FLAG_THRESHOLD`, for example. Each event fires the agent once with that event's context.

### What stops an agent firing

A subscribed trigger event does **not** fire the agent if any of the following hold:

- The agent's [status](#status-states) is **Disabled**.
- The agent's [URL or locale scope](#scope-url-locale) does not match the triggering comment.
- The agent's [daily, monthly, or rate-limit budget](#budgets-overview) is exhausted - the trigger is recorded as **dropped** with a reason. See [Drop Reasons](#drop-reasons).
- Concurrency for this agent is saturated (capped per-agent).
- The agent's tenant has invalid billing.
- The triggering action was itself made by a bot or another agent (loop prevention).
- The trigger was for a comment that has already been processed by this agent within the deduplication window.

When a subscribed trigger fires successfully, the agent's [Run History](#run-history) shows a row with status **Started** that transitions to **Success** or **Error** when the run completes.

### Vote and flag thresholds

Two triggers - **Comment Crosses Vote Threshold** and **Comment Crosses Flag Threshold** - require a numeric threshold on the edit form. The trigger fires the moment the count crosses the configured value (specifically, the flag-threshold trigger fires when `flagCount === flagThreshold`, so picking 1 means "fire on the first flag", and picking 5 means "fire when the fifth flag arrives").

### Deferred triggers

Any trigger can be deferred so the agent runs later, for example after votes/flags/replies have had time to settle. See [Deferred Triggers](#trigger-deferred-delay).

### Loop prevention

To prevent infinite loops comments **written by an agent** carry a `botId`. New-comment triggers ignore comments with a `botId`.

The net effect: agents can act in response to *human* actions in your tenant, but agent-sourced actions never fire any agent triggers. This
applies to all trigger types.

### REPLAY: the internal trigger

There is also an internal `REPLAY` trigger type used by the [Test Runs (Replays)](#test-runs-replays) feature. You cannot select it on the edit form - it exists so replay runs are tagged distinctly in run history and excluded from live-run views.
