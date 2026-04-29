By default an agent runs **immediately** after its trigger fires. The **Delay before running** field on the edit form changes that: the platform queues the trigger and runs the agent at the scheduled time.

### When to use a delay

- **Flag-threshold triggers** - flags often arrive in bursts. A 10-30 minute delay lets the picture settle so the agent acts on the final flag count rather than the moment-of-arrival.
- **Vote-threshold triggers** - same logic, particularly for downvote brigading.
- **Thread summarization** - the [Thread Summarizer template](#template-thread-summarizer) defaults to a 30-minute delay so it summarizes a conversation that has had time to develop, not a thread two replies in.
- **Cool-down / re-evaluation** - "24 hours after a comment is locked, consider whether to unlock it."

### Configuration

- **Field**: Delay before running.
- **Range**: 0 to 2,592,000 seconds (30 days).
- **Units**: Seconds, minutes, hours, or days.

### Idempotence

The deferred queue does not de-duplicate triggers. Two flags arriving 1 second apart on a 30-minute-delay agent will both schedule a run 30 minutes later, and the agent will run **twice**, both times against (mostly) the same context. If you want at-most-one-run-per-window semantics, the agent has to enforce it - typically by writing a [memory note](#tools-overview) on first run and checking for it on subsequent runs.

### Cost note

Deferred triggers are recorded **before** they run. A burst of triggers on a high-delay agent can pile up in the queue without spending tokens; the cost is paid only when the cron dispatches them. Use [Run History](#run-history) and [Drop Reasons](#drop-reasons) to see how often deferred triggers actually execute vs. get dropped at run-time for budget reasons.

### Replay does not honor delay

The [Test Runs (Replays)](#test-runs-replays) feature runs the agent immediately against historical comments - it does not wait for the configured delay. Treat that as a feature: replays are about previewing what the agent **would do** given context, not about reproducing real-time scheduling.
