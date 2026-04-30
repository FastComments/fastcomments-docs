FastComments ships five starter templates so you do not have to write a working agent from scratch. They are reachable from the [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) by clicking **Browse templates**.

When you pick a template:

1. The agent is created with **Status: Dry Run** and an internal name based on the template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). If that name is taken on your tenant, a numeric suffix is added.
2. You land directly on the edit form with everything pre-filled - prompt, triggers, allowed actions, and any thresholds. A banner across the top reads "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nothing is enabled yet. The agent will not act until you save and either keep dry-run on (to observe) or flip to Enabled.

### The five templates

- **[Moderator](#template-moderator)** - reviews new and flagged comments, warns first-time offenders, escalates to ban only after a warning. Triggers on new comments and on flag-threshold crossings (default flag threshold: 3). Allowed tools: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - replies warmly to first-time commenters with a short, personal welcome. Triggers on new-user-first-comment. Allowed tool: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - pins substantive top-level comments once they cross a vote threshold (default: 10), unpinning the previously pinned comment first. Triggers on vote-threshold crossings. Allowed tools: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - posts a neutral, single-paragraph summary on long threads after a delay, then pins it. Triggers on new comments with a 30-minute deferral so the thread settles before summarizing. Allowed tools: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - watches comment edits for mid-thread rewrites that warp replies, restores the original text, and DMs the author. Triggers on comment edits. Allowed tools: `edit_comment`, `warn_user`, `send_dm`.

### Customizing a template

Templates are starting points, not contracts. You are expected to:

- Tweak the **Initial prompt** to match your community voice.
- Add or remove **Triggers** to fit how often the agent should run.
- Add **Approvals** for any sensitive action - we strongly recommend gating `ban_user` behind approval for moderator-style templates.
- Add **Community guidelines** so the agent applies your written policy consistently. See [Community Guidelines](#community-guidelines).
- Set per-agent **Budgets** appropriate to how many triggers you expect.

The template is just a vehicle that pre-fills sensible defaults; once saved, the agent is yours.
