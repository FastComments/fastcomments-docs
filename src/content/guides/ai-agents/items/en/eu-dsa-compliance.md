FastComments enforces Article 17 of the EU Digital Services Act for tenants in the EU region: **fully-automated user suspensions are not permitted**.

### What that means in practice

When your tenant is in the EU region, on the agent edit form:

- The **Approvals** checkbox for `ban_user` is **locked on** and cannot be unticked.
- The label reads: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- A tooltip on the approval column reads: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Whatever else you configure, every `ban_user` call from any agent on a EU-region tenant goes to the [approvals inbox](#approval-workflow) for human review. The ban does not happen until a human approves it.

### Why this is enforced at the platform level, not the prompt level

System prompts can be ignored or worked around by a sufficiently misbehaving model. Article 17 compliance is too important to rely on the model's good behavior; it has to be a hard server-side gate that the tool dispatcher itself enforces. Which is what we do.

### What does and does not go through approval

- **`ban_user`**: always gated in the EU. Including:
  - Visible bans (`shadowBan: false`).
  - Shadow bans (`shadowBan: true`).
  - Bans with `deleteAllUsersComments: true`.
  - Bans with `banIP: true`.
- All ban variants land in the approvals inbox with the agent's reasoning and confidence; a human approves or rejects.

The other agent tools (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) are **not** affected by Article 17. You can still automate them. Article 17 is specifically about user suspensions.

### What about non-EU tenants

The lock does not apply outside the EU region. You can choose to gate `ban_user` behind approval anyway - we strongly recommend it for the first weeks of any moderation agent's life - but it is not enforced.

### Shadow bans

Shadow bans count as suspensions for Article 17 purposes (the user can post but their content is hidden). They are gated identically to visible bans.

### Region detection

Region is determined at the process level by the `REGION` environment variable on the FastComments deployment (read by `isEURegion()` in `models/constants.ts`). There is no per-tenant region field - the lock applies to every tenant on an EU-deployed instance. If you migrate your data from a non-EU deployment to an EU deployment, the lock takes effect for all tenants on that instance.

### What if all reviewers are unavailable

The approval will sit in the inbox until decided. It auto-expires 90 days after creation. There is no "no reviewer available, fall through to automated decision" path - that would defeat the point of Article 17.

If your community is so high-volume that EU bans cannot be reviewed in a reasonable time, consider:

- Adding more reviewers (see [Approval Notifications](#approval-notifications)).
- Switching the agent to use [`warn_user`](#tool-warn-user) more aggressively, since warnings are not subject to Article 17.
- Lowering the agent's appetite for banning by tightening the [community guidelines](#community-guidelines) or [initial prompt](#personality-prompt).

### See also

- [Tool: ban_user](#tool-ban-user) for what `ban_user` does and the destructive options behind extra opt-ins.
- [Approval Workflow](#approval-workflow) for the full approval lifecycle.
