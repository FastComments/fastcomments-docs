Fires when a user posts their first comment on this site (your tenant). This is **once per user** - subsequent comments from the same user do not refire it.

### Context the agent receives

- The new comment.
- Optional thread / user history / page context as configured.

When user history context is on, the user's recent comments list will of course be empty (or contain only this one), but the trust factor and account age are populated.

### Notable

- "First comment on this site" is scoped to the **tenant**, not site-wide across FastComments. A user with comments on other FastComments sites still fires this trigger the first time they post on yours.
- The trigger only fires for users with a userId. Anonymous unverified comments without a stable userId do not fire it.
- The trigger fires when the comment is approved/visible (not at initial post time). Edits or moderator actions on first comments do not refire it.

### Common uses

- **Welcome greeting** - the [Welcome Greeter template](#template-welcome-greeter) is built around this trigger.
- **Onboarding** - send a [DM warning](#tool-warn-user) (used here as a friendly heads-up rather than a warning) pointing the user at the community guidelines.
- **Reviewer notification** - if you want a human to look at every new commenter's first post, [`mark_comment_reviewed`](#tools-overview) can flag them for review.
