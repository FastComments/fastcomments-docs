Fires when a comment is pinned.

### Context the agent receives

- The pinned comment.
- Optional thread / user history / page context as configured.

### Who fires this

- A moderator clicking the pin action on the moderation page or comment widget.
- An agent calling [`pin_comment`](#tools-overview).

Loop prevention: agent-sourced pin events do not fire any agent triggers. A pin performed by an agent short-circuits all agent dispatch on that event, not just the originating agent.

### Note on the pair

Pin and unpin events are separate triggers. Subscribe to both if you want symmetric behavior. See [Trigger: Comment Unpinned](#trigger-comment-unpin).
