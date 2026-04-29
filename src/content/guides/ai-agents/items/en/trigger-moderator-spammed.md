Fires when a moderator marks a comment as spam.

### Context the agent receives

- The comment, with the post-action `Is Spam` flag.
- The **triggering user ID** - the moderator who acted.
- Optional thread / user history / page context as configured.

### Who fires this

A human moderator action. Agent-sourced spam marks (via [`mark_comment_spam`](#tools-overview)) do **not** fire this trigger.

### Common uses

- **Memory recording** - have an agent save a memory note about the spammed user (e.g., "previously spammed for X by moderator") so future moderation agents have context.
- **User-level enforcement** - a moderator marking a comment as spam might be the cue for an agent to also issue a warn or short ban, gated behind approval.
- **External system mirror** via [Webhooks](#webhooks-overview).
