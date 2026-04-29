**Template ID:** `top_comment_pinner`

The Top Comment Pinner watches for top-level comments that cross a vote threshold and pins them - replacing whatever was pinned previously on the same thread.

The built-in prompt instructs the agent to skip replies (pinning works on threads, so pinning a reply is rarely useful) and to filter out promotional content (so the agent does not boost popular link-spam).

### Triggers

- **A comment crosses a vote threshold** (`COMMENT_VOTE_THRESHOLD`, default vote threshold: 10).

The trigger fires when the comment's net votes (`up - down`) reaches the configured threshold. Tune the number on the edit form based on how active your threads are - 10 is a sensible default for moderately active sites.

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pinning is non-destructive - it can be reversed instantly - so this template usually runs without approvals.

### Recommended additions before going live

- **Tick "Include parent comment and prior replies in the same thread"** in [Context Options](#context-options). Without thread context the agent cannot reliably tell if there is already a pinned comment to unpin.
- **Adjust the vote threshold** to your site. On busy threads 10 happens too often; on quiet threads 10 may never happen.
- **Consider scoping by URL** if you only want pinned comments on certain sections of your site - news threads, say, but not announcement threads.

### Note on duplicate pinning

The agent's prompt instructs it to unpin first before pinning, but if the model misses that step the platform itself does not enforce a one-pinned-per-thread rule (you can have multiple). If duplicate pinning is a problem on your site, gate `pin_comment` behind approval and review each one - or write a tighter prompt.
