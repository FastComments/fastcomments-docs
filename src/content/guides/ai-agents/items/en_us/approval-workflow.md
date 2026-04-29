An **approval** is a queued tool call that requires a human to approve or reject before the platform executes it.

### Configuring approvals

On the agent edit form, the **Approvals** section lists every tool the agent is allowed to invoke (the allowlist) and lets you tick the ones that must be reviewed by a human. Unticked tools execute immediately. Ticked tools are queued.

Disallowed tools are *refused outright*, not queued - approvals only apply to tools that are otherwise allowed.

### What happens when a gated action fires

1. The agent picks a tool call (e.g. `ban_user`) with arguments, justification, and confidence.
2. Instead of executing, the platform queues an approval in `PENDING` state with the tool name, arguments, justification, confidence, and a context snapshot describing the trigger that produced it.
3. Notifications go out to reviewers (see [Approval Notifications](#approval-notifications)).
4. The agent's run completes and is recorded - the action is shown with **Pending approval** in [Run Detail View](#run-detail-view).

### Reviewing approvals

The approvals inbox at [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) lists pending, approved, rejected, and execution-failed approvals. For each:

- **Tool name and arguments** - exactly what the agent wants to do.
- **Agent reasoning** - the justification the agent supplied.
- **Confidence** - the agent's self-rated confidence, 0.0 to 1.0.
- **Context snapshot** - the comment, page, and user the action targets.

Two buttons: **Approve** and **Reject**. Approve actually executes the tool; Reject discards.

### Approval status states

An approval moves through these states:

| State | Meaning |
|---|---|
| `PENDING` | Awaiting human decision. |
| `APPROVED` | A human approved and the action ran. |
| `REJECTED` | A human rejected. The action did not run. |
| `EXECUTION_FAILED` | A human approved but the action failed (e.g., the target comment was already deleted). |
| `EXECUTING` | Transient: a human clicked Approve and the action is running. Used to serialize concurrent approve clicks so a tool with real side effects never runs twice. |

The `EXECUTING` state matters when two reviewers click Approve simultaneously - one wins, the other sees the approval has already moved.

### What gets cleaned up

Pending approvals stay pending until decided. They auto-expire **90 days** after creation. Approvals in any other state are also cleared after 90 days for storage hygiene.

The approval's "decided by" / "decided at" / "executed at" / "execution result" fields are populated as the approval moves through states - all visible in the inbox detail view.

### Webhook integration

Two webhook events fire as approvals move:

- **`approval.requested`** - on PENDING insert.
- **`approval.decided`** - on transition to APPROVED, REJECTED, or EXECUTION_FAILED.

Both are signed like every other webhook. See [Webhook Events](#webhook-events) and [Webhook Payloads](#webhook-payloads).

### Hardening: known-tool gate

Approvals refuse to enqueue any tool name that is not a recognized agent tool. This is a defense-in-depth check: even if a future code path passes an LLM-derived tool name into the approval flow, that string can never land as a queued approval. The set of known tool names is the same set listed in [Tools Reference](#tools-overview).

### Common gating patterns

- **Brand-new moderation agent** - gate `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Watch the inbox for a few weeks, then drop gating from low-error tools.
- **Long-term moderation agent** - keep `ban_user` and any irreversible actions (`deleteAllUsersComments`, `banIP`) gated forever.
- **EU region** - `ban_user` is locked on by Article 17 regardless of what you tick. See [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### What approvals do **not** do

- They do not delay the agent's other tool calls. The agent's run can call several tools, and only the gated ones queue - the rest execute as normal.
- They do not roll back the agent's run if the human rejects. The non-gated portion of the run is already done.
