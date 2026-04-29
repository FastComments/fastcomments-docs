Fires when a comment's net vote count reaches the configured threshold. Net votes is `votesUp - votesDown`.

### Required configuration

- **Vote threshold** - integer >= 1. The trigger fires on the vote that brings net votes to exactly this number.

If the threshold is 10 and a comment goes from 9 to 10 net votes, the trigger fires once. If a vote then takes it from 10 to 11, the trigger does **not** fire again - it does not re-fire on every additional vote past the threshold.

### Context the agent receives

- The comment, with current vote counts.
- The **vote direction** (`up` or `down`) of the vote that triggered the threshold crossing.
- Optional thread / user history / page context as configured.

### Notable

- A comment that goes up to 10, drops back to 9, and rises to 10 again will fire the trigger twice. There is no per-comment "fired once" state - if you need that semantics, have the agent record a [memory note](#tools-overview) on first run and check for it on subsequent runs.
- Threshold is always a **net** vote count, not raw upvotes. A comment with 12 up and 2 down has net 10 and fires the trigger; one with 10 up and 0 down also fires.
- Down-vote-only crossings are possible - a comment going from 11 to 10 because of a down-vote also fires. The `voteDirection` parameter in the context tells the agent which direction the threshold crossing came from.

### Common uses

- **Pinning** - the [Top Comment Pinner template](#template-top-comment-pinner) is built around this trigger.
- **Promotion / featured comment workflows** - emit an event via [Webhooks](#webhooks-overview) so an external system can promote the comment elsewhere on your site.
- **Engagement tracking** - record a memory note about the user who wrote the comment so other agents know they have produced popular content.

### Tuning

The right threshold is community-specific. Watch [Run History](#run-history) for a few days at a low threshold (5) to see how often it fires. Raise the threshold until the firing rate matches the cadence you actually want.
