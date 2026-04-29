Fires when a moderator approves a comment.

### Context the agent receives

- The newly-approved comment.
- The **triggering user ID** - the moderator who approved.
- Optional thread / user history / page context as configured.

### Who fires this

A human moderator action.

### Notable

- An "approved" comment is a **visible** comment in FastComments terminology. See [How Approvals Work](/guide-moderation.html#moderation-approvals) in the moderation guide for the distinction between approved/unapproved and reviewed/unreviewed.
- The trigger fires on approval **transitions**: a comment going from unapproved to approved fires it; a comment that was already approved being re-saved does not.
- For tenants where comments default to auto-approved, this trigger fires only when a moderator explicitly re-approves a previously-hidden comment.

### Common uses

- **Welcome / engagement** - an agent can reply to first-time commenters at the moment a moderator approves them, rather than at post time.
- **Cross-agent coordination** - if a separate agent had marked the comment for review, the approval is the cue that human review is finished.
- **Audit trail** via [Webhooks](#webhooks-overview).
