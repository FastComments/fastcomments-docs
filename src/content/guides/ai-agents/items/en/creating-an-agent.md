From the [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) you can create an agent in two ways:

- **From a template.** Click **Browse templates** and pick one of the four built-in starter agents. The form lands pre-filled and the agent's status is **Dry Run**. See [Starter Templates](#starter-templates).
- **From scratch.** Click **Create new agent**. The form lands blank.

Either way, the same edit form is what you save and edit afterwards. This page walks the form top to bottom.

### Basics

- **Internal name.** A short identifier used only in admin dashboards (run history, analytics, audit logs). Lowercase with underscores works well: `moderator`, `welcome_greeter`. If a template's internal name is already taken, the form auto-suffixes (`tos_enforcer_2`, etc.).
- **Display name.** Shown publicly whenever the agent posts a comment. This is what your readers see.
- **Status.** Disabled, Dry Run, or Enabled. New agents always default to Dry Run. See [Status States](#status-states).

### Model

Pick the LLM. See [Choosing a Model](#choosing-a-model).

### Budget

Optional daily and monthly caps in your account currency, plus an **Alert thresholds** checklist (default 80% and 100%). See [Budgets Overview](#budgets-overview) and [Budget Alerts](#budget-alerts).

### Personality

The **Initial prompt** is the system prompt that defines tone, role, and decision rules. Plain text, no template syntax. See [Personality and the Initial Prompt](#personality-prompt).

### Context

The Context fieldset has three checkboxes, a guidelines text area, and the scope inputs:

- Include parent comment and prior replies in the same thread.
- Include the commenter's trust factor, account age, ban history, and recent comments.
- Include page title, subtitle, description, and meta tags.
- An optional **Community guidelines** text block that gets prepended to every prompt.
- **Restrict to specific pages** - URL pattern allowlist (one per line). Empty means tenant-wide.
- **Restrict to specific locales** - locale allowlist via a dual-list picker. Empty means every locale.

More context produces better decisions but raises token cost per run. See [Context Options](#context-options), [Community Guidelines](#community-guidelines), and [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Pick at least one event from the list. For vote-threshold and flag-threshold triggers you must also set the threshold. The optional **Delay before running** field defers execution after a trigger fires (useful for flag thresholds where votes are still settling). See [Trigger Events Overview](#triggers-overview) and [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Tick **Allow any tool calls** to expose the full tool palette. Otherwise tick the specific tools the agent is allowed to use - disallowed tools are pruned from the model's palette and refused at dispatch time. The **Ban options** sub-section gates the destructive ban variants (delete-all-comments, ban-by-IP) behind explicit opt-ins. See [Allowed Tool Calls Overview](#tools-overview) and [Tool: ban_user](#tool-ban-user).

### Approvals

Tick the actions that must be approved by a human before the agent executes them. Approvals only apply to tools the agent is allowed to invoke; disallowed tools are refused outright. In the EU region, **ban_user** is locked on by Article 17 of the Digital Services Act. See [Approval Workflow](#approval-workflow) and [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Approval notifications

If approvals are enabled, choose who is emailed:

- **All admins and moderators** - account owners, super admins, and comment moderator admins.
- **Specific users** - hand-picked from a dual-list picker.

Each reviewer's individual delivery frequency (immediate, hourly digest, daily digest) is set on their own profile. See [Approval Notifications](#approval-notifications).

### Stats

Read-only. Total runs, last run timestamp, and the ID of the most recent comment the agent wrote (if any).

### Save

Click **Save agent**. The page redirects back to the agent list. New agents are immediately eligible to receive triggers in dry-run.

### Editing later

Each row on the agent list page exposes per-agent actions: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, and **Delete**. Editing an agent does not retro-affect already-recorded runs - history is preserved. Replay snapshots also freeze the agent's config at the point the replay was started, so a saved replay's results stay reproducible even after you edit the prompt.
