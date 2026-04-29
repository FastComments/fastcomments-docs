An **AI Agent** is an autonomous worker, scoped to your FastComments tenant, that watches for events in your community and takes action on your behalf.

Each agent has three things you control:

1. **A personality.** A free-text initial prompt that defines tone, role, and decision-making style ("You are a warm community greeter", "You enforce the community rules but err toward warning over banning", and so on).
2. **One or more triggers.** A list of events that wake the agent up - a new comment, a comment crossing a vote or flag threshold, a moderator action, a user's first comment on the site, and others. The full list is in [Trigger Events Overview](#triggers-overview).
3. **An allowlist of tools.** What the agent is allowed to do - post a comment, vote, pin, lock, mark spam, ban a user, warn via DM, award a badge, send email, save and search a shared memory. The full list is in [Allowed Tool Calls Overview](#tools-overview).

When a trigger fires, the agent receives a context message describing what happened (the comment, the page, optional thread/user/page context) and is prompted with its initial prompt and your community guidelines. It then calls tools to act, recording a justification and a confidence score with every call.

### Agents run asynchronously

Agents **never block the user action that triggered them**. A reader posts a comment, the comment is saved and shown to the thread, the response is returned, and only *then* does the agent run on it - either immediately or after a configured delay (see [Deferred Triggers](#trigger-deferred-delay)). Nothing the agent does adds latency to the user-facing experience.

### Why use them

- **Moderate at scale.** Mark obvious spam and ban repeat offenders without watching the queue around the clock.
- **Welcome new commenters.** Reply to first-time commenters in your voice.
- **Surface the best content.** Pin substantive top-level comments once they cross a vote threshold.
- **Enforce your guidelines consistently.** Apply the same policy text to every borderline comment.
- **Summarize long threads.** Post neutral summaries of multi-page debates.

### What keeps you in control

- **Dry-run mode.** Every new agent ships in **Dry Run**: it processes triggers, runs the model, and records what it *would* do, but takes no real actions. See [Dry-Run Mode](#dry-run-mode).
- **Approvals.** Any subset of actions can be gated behind human approval. See [Approval Workflow](#approval-workflow).
- **Per-agent and per-account budgets.** Hard daily and monthly caps. See [Budgets Overview](#budgets-overview).
- **Tool allowlist.** Disallowed tools are pruned from the model's palette - the agent literally cannot request them. See [Allowed Tool Calls Overview](#tools-overview).
- **Audit fields on every action.** The model must include a justification and confidence score. Both appear in the run timeline and on every approval. See [Run Detail View](#run-detail-view).
- **EU DSA Article 17.** In the EU region, fully-automated bans are blocked. See [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **No training on your data.** FastComments uses providers that do not train on your prompts or comments.

### Where they fit alongside human moderation

Agents and human moderators share the same comment platform: agents take actions through the same channels (approve, spam, ban, badge, pin, lock, write) and those actions appear in the same [Comment Logs](/guide-moderation.html#comment-logs), the same [Moderation Page](/guide-moderation.html#moderate-comments-page), and the same notification streams. Agents and humans see each other's work and can each react to the other - moderator actions are themselves valid agent triggers (see [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) and friends).
