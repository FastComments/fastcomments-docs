Bir moderatör bir kullanıcıya rozet verdiğinde tetiklenir.

### Context the agent receives

- The **badge ID** of the badge awarded.
- The **triggering user ID** - the moderator who awarded the badge.
- Optional thread / user history / page context as configured.

The fire-site does **not** include a `commentId` in the trigger payload, even if the badge was awarded with respect to a specific comment.

### Who fires this

Bir insan moderatör işlemi.

### Notable

- The **badge ID** alone is included; the agent does not receive the badge metadata (name, image). If the agent needs to reason about *which* badge was awarded, embed that context in the [başlangıç istemi](#personality-prompt) or [topluluk yönergeleri](#community-guidelines).
- The trigger fires once per badge award, not per user. Awarding the same badge to a user twice fires it twice (each award is a distinct event).

### Common uses

- **Reciprocal recognition** - an agent can post a "harika katkı için teşekkürler" reply when a specific badge is awarded.
- **External recognition workflow** via [Webhook'lar](#webhooks-overview) - mirror badge awards into your own user-engagement system.
- **Memory recording** - "bu kullanıcı tanınmış bir katkıcıdır" notes that future moderation agents should weight in their decisions.

---