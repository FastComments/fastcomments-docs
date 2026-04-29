Fires when a comment is deleted.

### Context the agent receives

- The comment that was just deleted - text, author, page.
- Optional thread / user history / page context as configured.

### Notable

- Fires for both **soft deletes** (where the comment is hidden but retained for audit) and **hard deletes** (where the comment is fully removed). The trigger handler resolves the comment from the cascade delete pipeline; what the agent sees is the last known state.
- Once a comment is fully deleted, tools that target it (`pin_comment`, `mark_comment_spam`, etc.) on that comment ID will fail.

### Common uses

- **Audit forwarding via [Webhooks](#webhooks-overview)** - emit a `trigger.succeeded` event so an external system records what was deleted.
- **Memory writes** - have the agent record a [memory note](#tools-overview) about a deletion pattern (deleted comment was the user's third in 24 hours, etc.).
- **Cross-thread effects** - notice when a deletion changes the structure of a thread the agent has previously summarized, and consider whether to re-summarize.

### Cost-of-operating note

If you have a high-deletion-volume site (heavy human moderation), this trigger can fire often.
