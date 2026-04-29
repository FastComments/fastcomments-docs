This is the five-minute path from "we have AI Agents" to "an agent is responding to live traffic, gated by approvals." If you want the long form, every step links to the page that covers it in depth.

### 1. Open the AI Agents page

Go to [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) in your account. The first time you land here you will see either:

- A blank-state with a **Browse templates** and **Start from scratch** button (you have agents available to create), or
- An upsell page if your plan does not include agents - see [Plans and Eligibility](#plans-and-eligibility).

### 2. Pick a starter template

Click **Browse templates**. Pick one of:

- [Moderator](#template-moderator) - reviews flagged or new comments, warns first-timers, escalates to ban only after a warning.
- [Welcome Greeter](#template-welcome-greeter) - replies to first-time commenters.
- [Top Comment Pinner](#template-top-comment-pinner) - pins substantive comments once they cross a vote threshold.
- [Thread Summarizer](#template-thread-summarizer) - posts a neutral summary on long threads.

Each template lands on a pre-filled edit form with **Status: Dry Run** already selected.

### 3. Review and save

On the edit form, do at minimum:

- **Internal name.** A short identifier used in admin dashboards.
- **Display name.** What appears publicly when the agent posts a comment.
- **Initial prompt.** Edit the template's prompt to match your voice and your specific rules.
- **Approvals.** Tick the actions that should require human review before they take effect. We recommend at least `ban_user` for any moderation-style agent. See [Approval Workflow](#approval-workflow).

Click **Save agent**.

### 4. Watch it in dry-run

The agent is now live in **Dry Run**. It will receive its triggers, call the model, and record actions on the [Run History](#run-history) page - with the **Dry Run** badge on each row - but it does not take real actions. Visit a few of the run details (see [Run Detail View](#run-detail-view)) and look at:

- The actions the agent picked.
- The justification and confidence on each action.
- The full LLM transcript.

If the agent is making decisions you disagree with, edit the initial prompt or tick more approvals.

### 5. Run a test against past comments

From the agents list page, click **Test run** on the agent's row. The form has a single **Days** numeric input (1 to 90). Sample size and the hard cap on comments evaluated are shown informationally - they are computed server-side, not user-set. The replay runs against historical comments without taking real actions and reports what the agent **would** have done versus what actually happened (was the comment later approved, marked spam, deleted, and so on). See [Test Runs (Replays)](#test-runs-replays).

### 6. Flip to Enabled

When you are happy with the dry-run and replay output, edit the agent and change **Status** to **Enabled**. From here on, real actions land. The Run History page now shows live runs without the dry-run badge, and any action you marked for approval appears in the [approvals inbox](#approval-workflow).

### What's next

- Set [Budgets](#budgets-overview) and [Budget Alerts](#budget-alerts).
- Configure [Webhooks](#webhooks-overview) if you want external systems to react to agent events.
- Add [Community Guidelines](#community-guidelines) to keep agent decisions aligned with your written policy.
