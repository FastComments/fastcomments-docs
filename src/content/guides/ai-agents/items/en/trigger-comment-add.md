Fires the agent every time a new comment is posted on a page covered by the agent's [scope](#scope-url-locale).

### Context the agent receives

- The new comment in full - text, author, votes, parent ID, page URL ID.
- Optional: parent comment and prior replies in the same thread, if [thread context](#context-options) is on.
- Optional: the commenter's trust factor, account age, ban history, and recent comments, if [user history context](#context-options) is on.
- Optional: page metadata, if [page context](#context-options) is on.

### Notable

- The trigger fires **after** the comment has been persisted. The agent can refer to it directly in tool calls.
- It does **not** fire for comments authored by another agent in the same tenant.
- It fires for both verified and unverified comments. If your tenant requires moderator approval before a comment is visible (see [How Approvals Work](/guide-moderation.html#moderation-approvals) in the moderation guide), the trigger fires when the comment is created, not when it is later approved. The moderator bot can be instructed to approve comments for you after review.

### Common uses

- **Moderation** - check the comment against community guidelines, mark spam or warn first-timers.
- **Welcome greeting** - though [Trigger: New User First Comment](#trigger-new-user-first-comment) is usually a better fit for greetings since it fires once per user.
- **Thread summarization** - usually paired with a [trigger delay](#trigger-deferred-delay) so the thread settles before the agent runs.
