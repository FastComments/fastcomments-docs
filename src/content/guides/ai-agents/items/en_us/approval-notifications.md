When the agent queues an approval, the platform notifies reviewers via email. Two settings on the edit form control this: **who** is notified and **how often**.

### Who: notify mode

Two modes:

- **All admins and moderators** (default) - every account owner, super admin, and comment moderator admin on the tenant is a candidate reviewer.
- **Specific users** - hand-pick a list from a dual-list picker on the edit form.

Either way, a candidate reviewer must have an account on the tenant and a valid email address to receive notifications.

### How often: per-user frequency

Each candidate reviewer's **own profile** sets their personal notification frequency for agent approvals:

- **Immediate** (default) - one email per pending approval, sent as soon as the approval is created.
- **Hourly** - one digest email per hour summarizing all approvals queued in that hour.
- **Daily** - one digest email per 24 hours.
- **Disabled** - no emails. The user can still review approvals via the inbox UI; they just are not pinged.

The user changes this setting on their own profile, not on the agent edit form. This is intentional - one tenant might have ten agents, and a moderator should not have to set their preferred frequency on every agent independently.

### Cron jobs that drive digests

- **`hourly-agent-approval-digest`** - sweeps every hour, batches approvals queued since each user's last digest, sends one email per user.
- **`daily-agent-approval-digest`** - same, daily.
- **`agent-approval-reaper`** - prunes approvals that have aged past 90 days regardless of state.

The hourly and daily digest crons are scoped per-recipient: a user with hourly frequency is processed by the hourly cron and skipped by the daily one (and vice versa). Immediate-frequency users are notified by the approval-create code path, not by the crons.

### Dedup state

The platform tracks which users have already been emailed about each approval. Once a user has been notified (immediately or in a digest), they will not be emailed again for the same approval - even if they change their frequency from immediate to daily mid-cycle.

### Approving from the email

Each notification email contains a one-click signed login link that takes the reviewer directly to the approval detail page, already authenticated. They can approve, reject, or open the [Refine Prompts](#refining-prompts) flow from there.

### What if no admins exist

If `notifyMode` is `All admins and moderators` but the tenant has no super admins, comment moderator admins, or account owners with valid emails, the platform logs a warning and the approval still queues - just nobody gets notified about it. It will sit in the inbox until someone happens to look.

If `notifyMode` is `Specific users` but you have not selected any users, same outcome.

### What if billing notifications are disabled

[Budget Alerts](#budget-alerts) - the budget-related emails - go to billing admins **regardless of the per-user notification preference**. This is intentional: budget overruns affect cost, and the billing owner needs to know.

Approval notifications honor only the per-user agent-approval frequency setting. They do not check the broader admin-notifications opt-out - a user who has opted out of admin notifications will still receive approval emails if they are on the reviewer list, unless their agent-approval frequency is set to **Disabled**.

### See also

- [Approval Workflow](#approval-workflow) for the full lifecycle of an approval.
- [Refining Prompts](#refining-prompts) for the "I keep approving the same kind of mistake" workflow.
