The Edit tool lets the agent replace the text of an existing comment. It is destructive in a way most other moderation tools are not: it overwrites user-authored content. Reserve it for narrow, clear-cut cases.

### What it does

The agent passes a comment ID and a replacement body. The platform writes the new text to the comment and records a `TextChanged` entry in the comment's audit log capturing both the old text and the new text. The original is never lost - moderators can read what the comment said before the agent edited it.

The replacement runs through the same rendering pipeline as a human edit: profanity masking, mention parsing, hashtag extraction, and link/image handling all behave exactly as if the original author had submitted the new text.

### Scope

Like every comment-mutating tool, Edit is constrained to the trigger's allowlist - the agent can only edit the comment the trigger fired on, its parent, or another in-scope comment from the same trigger context. A prompt-injection attempt to "edit comment XYZ" where XYZ is unrelated will be refused server-side before the executor runs.

### Loops

When the agent edits a comment, the platform fires a `COMMENT_EDIT` trigger as it would for a human edit, but **suppresses dispatch to other agents**. This prevents two agents that both listen for `COMMENT_EDIT` from ping-ponging on each other's edits.

### When to allow it

For agents that handle PII redaction, or for self-editing summarizer/digest agents. Most moderation agents do **not** need this tool - mark-spam, warn, and ban cover the typical lifecycle.

### Approvals

**Strongly consider gating behind approval**, especially while you are building trust in the agent. An agent rewriting a user's words is the kind of action a community will notice and react to, and it is harder to "undo" reputationally than a deletion.

### See also

- [Trigger: Comment Edited](#trigger-comment-edit) - the trigger fired when a comment's text changes.
- [Approval Workflow](#approval-workflow) - how to gate the tool behind human review.
