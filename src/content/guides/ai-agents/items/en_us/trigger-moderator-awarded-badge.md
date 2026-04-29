Fires when a moderator awards a badge to a user.

### Context the agent receives

- The **badge ID** of the badge awarded.
- The **triggering user ID** - the moderator who awarded the badge.
- Optional thread / user history / page context as configured.

The fire-site does **not** include a `commentId` in the trigger payload, even if the badge was awarded with respect to a specific comment.

### Who fires this

A human moderator action.

### Notable

- The badge ID alone is included; the agent does not receive the badge metadata (name, image). If the agent needs to reason about *which* badge was awarded, embed that context in the [initial prompt](#personality-prompt) or [community guidelines](#community-guidelines).
- The trigger fires once per badge award, not per user. Awarding the same badge to a user twice fires it twice (each award is a distinct event).

### Common uses

- **Reciprocal recognition** - an agent can post a "thanks for the great contribution" reply when a specific badge is awarded.
- **External recognition workflow** via [Webhooks](#webhooks-overview) - mirror badge awards into your own user-engagement system.
- **Memory recording** - "this user is a recognized contributor" notes that future moderation agents should weight in their decisions.
