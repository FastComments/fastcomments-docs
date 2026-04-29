Clicking **View** on a row in [Run History](#run-history) opens the per-run detail page. This is where you read the agent's reasoning and judge its decisions.

### Top: run summary

- **Agent** - which agent ran.
- **When** - timestamp.
- **Status** - Started / Success / Error, plus the **Dry Run** badge if applicable.
- **Cost** - per-run cost in your tenant's currency.
- **Cost per action** - cost divided by the count of non-pending actions, useful for spotting unusually expensive runs.

### Actions taken

A list, in order, of every tool call the run made. Each entry shows:

- **Action label** - "Wrote a comment", "Marked a comment as spam", "Banned a user", and so on. The label maps from the action type enum.
- **Reference ID** - the affected comment, user, or badge ID, shown as monospaced text (not a hyperlink).
- **Agent reasoning** - the justification the agent supplied with the call.
- **Confidence** - the agent's self-rated confidence, displayed as a percentage.
- **Pending approval** badge - if the action is queued in the [approvals inbox](#approval-workflow) instead of executed.

If the run took zero actions, the section reads: "No actions were taken during this run."

### LLM transcript

Below actions, the full transcript of the agent's conversation with the LLM:

- **System** - the system prompt (platform suffix + your initial prompt + community guidelines).
- **User** - the context message describing the trigger.
- **Assistant** - the model's responses, including tool calls.
- **Tool** - the tool result fed back to the model (e.g., what `search_memory` returned).

Long messages are collapsible; click **Expand** / **Collapse** to view.

### Reading transcripts

The transcript is the most important page for tuning. When the agent makes a decision you disagree with, read it back to see:

- What the model **saw** (the User context message).
- What the model **decided** (the Assistant tool calls).
- What the model **considered** (any tool results - e.g., did the agent actually call `search_memory` and did it find anything before banning).

If the model is consistently making the same kind of mistake, edit the [initial prompt](#personality-prompt) - or use [Refining Prompts](#refining-prompts) from a rejected approval.

### Action references

The reference IDs are shown as monospaced text (not hyperlinks):

- Comments: the comment ID.
- Users: the user ID.
- Badges: the badge ID.

You can copy the ID to look up the affected record in the relevant moderation/admin page.

### What's missing in dry-run

Dry-run runs show the **same** actions, justifications, and confidence scores. The only difference is the **Dry Run** badge on the status row. The reference IDs for comments / users / badges are still shown - the agent just did not affect them.

### Errors

For runs in `Error` state, the detail page shows the underlying error message. Common errors:

- **No LLM API key configured** - tenant or platform misconfiguration.
- **LLM call timeout** - the LLM provider was slow or unavailable.
- **Tool dispatch failure** - the agent picked a tool with bad arguments (e.g., a comment ID that no longer exists).
- **Budget exhausted mid-run** - the agent's cap was hit while the run was in flight. The run was halted.

Errors do not roll back partial actions - any tool calls completed before the error remain.
