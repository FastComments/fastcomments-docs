**Template ID:** `gaslight_detector`

The Gaslight Detector watches for comment edits that rewrite history in the middle of a conversation - the kind where an author changes the meaning of an earlier comment after replies have been written, leaving downstream replies looking out of context or wrong. When the agent decides an edit crosses that line, it restores the original text and DMs the author to explain.

This is a higher-risk template because it modifies user content. Run it in [dry-run](#dry-run-mode) longer than you would a read-only template, and gate `edit_comment` behind [approval](#approval-workflow) until you trust the model's judgment on your traffic.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - the agent compares the new and previous text and decides whether the edit warps replies that already exist.

See [Trigger: Comment Edited](#trigger-comment-edit) for the full payload, including the previous comment text and reply count at edit time.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - used to restore the original text when the edit is judged to be gaslighting.
- [`warn_user`](#tool-warn-user) - issues a soft warning the user sees on their next visit.
- [`send_dm`](#tools-overview) - the explanation channel; the user gets a direct message describing why their edit was reverted.

It cannot ban, mark spam, vote, or post new comments - the surface is intentionally narrow.

### Recommended additions before going live

- **Gate `edit_comment` behind [approval](#approval-workflow).** Reverting a comment is visible to the author and to anyone who saw the edited version, so a false positive is embarrassing. Keep approvals on until dry-run shows the agent is consistent.
- **Tighten the prompt with what counts as gaslighting on your site.** The default prompt is short on purpose. Give the model concrete examples - "flipping a yes/no claim", "deleting a number that replies cite", "adding a hostile sentence after replies were posted" - and explicit non-examples like typo fixes, formatting cleanup, or adding sources.
- **Use the reply count from the trigger context.** Edits to comments with zero replies cannot warp a conversation; the prompt should tell the model to skip those.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** in [Context Options](#context-options). The model is far less aggressive when it can see a long-time good-faith account.
- **Consider a short edit-grace window in the prompt.** Many edits within the first 30-60 seconds are typo fixes; instruct the model to ignore edits that quick.

### Recommended dry-run window

Run for at least two weeks of real traffic in [dry-run](#dry-run-mode) before flipping to Enabled, and review every flagged edit during that window. Use [Test Runs (Replays)](#test-runs-replays) to replay the last 30 days of edits against the agent before going live.
