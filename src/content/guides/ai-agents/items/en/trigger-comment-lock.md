Fires when a comment is locked.

### Context the agent receives

- The locked comment.
- Optional thread / user history / page context as configured.

### Who fires this

- A moderator using the lock action on the moderation page or comment widget.

### Common uses

- **Notify reviewers** - a lock event often follows a heated thread; a webhook out to your moderation Slack channel can let humans pick up the rest.
- **Cool-down enforcement** - schedule a [deferred trigger](#trigger-deferred-delay) on a separate agent that, 24 hours after the lock, considers whether to unlock.

### Pair

See [Trigger: Comment Unlocked](#trigger-comment-unlock) for the mirror trigger.
