Fires when a comment's flag count reaches **exactly** the configured threshold.

### Required configuration

- **Flag threshold** - integer >= 1. The trigger fires the moment `flagCount === flagThreshold`. It does not fire again on subsequent flags past the threshold.

If the threshold is 3 and three users flag the comment, the agent fires once on the third flag. A fourth, fifth, or sixth flag does **not** re-fire it.

### Context the agent receives

- The flagged comment.
- Optional thread / user history / page context as configured.
- The flag count is in the comment block as `Flag Count: N`.

### Notable

- The trigger only fires on the comment crossing the threshold from below via the platform's flag-handling path (where `didIncrement === true`). Direct DB writes that set `flagCount` to the threshold value do not fire it; flags beyond the threshold do not refire it either.
- It does not include who flagged the comment - flags are anonymous to the agent. If you want to look at flagging users, fetch them from your own data.
- A trigger delay (see [Deferred Triggers](#trigger-deferred-delay)) is *strongly* recommended on this trigger - flags often arrive in bursts during a heated thread, and a small delay lets the picture settle before the agent acts.

### Common uses

- **Moderation review** - a flagged comment is the canonical "humans think this might be bad" signal. The [Moderator template](#template-moderator) subscribes to this trigger by default with a flag threshold of 3.
- **Pre-moderation queue augmentation** - the agent runs an initial pass and either marks the comment for moderation (with `mark_comment_reviewed`) or escalates further.
- **Anti-brigading** - combine this trigger with [user history context](#context-options) and let the agent see prior bans/duplicate-content signals before acting.

### Pair recommendations

Subscribe to **both** `COMMENT_ADD` and `COMMENT_FLAG_THRESHOLD` if you want a moderation agent that catches obvious cases on first sight and re-evaluates borderline ones once flags accumulate. The two events fire independently - the agent will run twice if both are subscribed and both fire, but the second run sees the now-flagged state.
