Fires when a moderator marks a comment as reviewed.

### Context the agent receives

- The comment.
- The **triggering user ID** - the moderator who reviewed.
- Optional thread / user history / page context as configured.

### Who fires this

A human moderator action on the moderation page, comment widget, or via API.

### Common uses

- **Audit forwarding** via [Webhooks](#webhooks-overview).
- **Memory writes** - record a memory note that this comment was human-reviewed so other agents do not double-process it.

### Notable

- "Reviewed" is one of the moderation queue states tracked separately from "approved" and "spam". A comment can be approved-and-reviewed, approved-but-not-reviewed, etc. See [How Approvals Work](/guide-moderation.html#moderation-approvals) in the moderation guide.
- This trigger is high-frequency on tenants with many moderators. Subscribe selectively and budget accordingly.
