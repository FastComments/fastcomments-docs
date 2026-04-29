An agent's **tools** are the actions it can take. The agent edit form has an **Allowed tool calls** section where you tick the tools this agent is allowed to use, and an **Approvals** section where you tick the actions that should require a human to approve before they take effect.

There are three levels for any tool:

- **Disallowed** - the agent cannot see or use it.
- **Allowed, no approval** - the agent uses it directly. Recorded in run history.
- **Allowed, with approval** - the agent's call is queued for human review and only runs when a human approves.

Disallowed tools are silent: the agent cannot ask for them and the platform refuses them outright. Approval-gated tools always go through the [approvals inbox](#approval-workflow).

### Audit trail on every action

Every action the agent takes is recorded with a short justification (1-2 sentences explaining why) and a confidence score (0.0-1.0). Both appear in [Run Detail View](#run-detail-view) and on every [approval](#approval-workflow). Searching memory is the one read-only exception: it is not recorded as an action and is always available regardless of the allowlist.

### Tool reference

#### Posting comments

Lets the agent post a comment as itself. The comment is shown publicly under the agent's display name. Used by greeter and summarizer agents. Reversible - any moderator can remove a bad comment. Usually allowed without approval; gate it if your community needs every public-facing message to be human-reviewed.

#### Voting on comments

Lets the agent vote up or down on a comment. The vote counts toward the comment's vote total like any other vote. Most communities prefer not to have bots voting; not enabled in any starter template. If you do allow it, voting is reversible.

#### Pin / unpin a comment

Lets the agent pin a comment to the top of the page or unpin one that is already pinned. The platform does not enforce a one-pin-per-thread rule, so a pinning agent should be instructed to unpin the previous pinned comment first. Used by the Top Comment Pinner template. Reversible; usually allowed without approval.

#### Lock / unlock a comment

Lets the agent prevent further replies under a comment, or restore replies. The locked comment stays visible. Useful for cool-downs on heated threads, paired with a deferred unlock. Reversible but visible to your community; consider gating behind approval on high-stakes communities.

#### Mark / unmark spam

Lets the agent mark a comment as spam (hiding it from readers and feeding the spam classifier) or clear that flag. The bread-and-butter tool for any moderation agent. Reversible. Strongly consider gating behind approval for the first weeks while you build trust in the agent.

#### Approve / un-approve a comment

Lets the agent show a held comment to readers, or hide an already-visible one. Most useful on tenants that hold new comments for moderator review. High stakes when un-approving a visible comment - consider gating behind approval.

#### Mark a comment reviewed

A queue-state tool: marks a comment as "a moderator (or agent) has looked at this." Does not change visibility. Low stakes; rarely gated.

#### Award a badge

Lets the agent give a user a badge from your tenant's badge configuration. Reversible by a moderator. Rarely gated. The agent must know the badge ID, so include the relevant IDs in your [community guidelines](#community-guidelines) or [initial prompt](#personality-prompt).

#### Send email

Lets the agent send a plain-text email from `noreply@fastcomments.com` to an address it picks. Use sparingly - email is the highest-friction tool and bad emails are hard to undo. Strongly consider gating behind approval, and route approval emails to whoever owns the inbox the agent will end up emailing.

#### Save / search agent memory

Two paired tools that read and write a shared notes pool about the user a trigger fired for. Memory is shared across all agents in your tenant, so a triage agent's notes inform a moderator agent's decisions. Search is read-only and always available; saving is rarely gated. See [Agent Memory System](#agent-memory-system) for the full design.

#### Warn a user

Sends a private DM warning to a user about a specific comment, and atomically records the warning in agent memory. The platform's escalation policy is built around this tool - warn first, ban only if the user reoffends. Less commonly gated than `ban_user`, but consider gating during the first weeks of an agent's life. See [Warn user](#tool-warn-user) for the full page.

#### Ban a user

The most consequential tool an agent can call. Bans a user with a fixed duration, optionally as a shadow ban, optionally also banning the IP, optionally also deleting all of the user's comments. The two destructive options (IP, delete-all) are gated behind extra opt-ins on the edit form. In the EU region, all bans require human approval (see [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Strongly consider gating behind approval everywhere. See [Ban user](#tool-ban-user) for the full page.

### Ban-tool sub-options

The Ban tool exposes two destructive options - delete-all-comments and ban-by-IP - that are hidden from the model entirely until you opt them in via the **Ban options** section on the edit form. Even if the model hallucinates the parameter, the platform refuses values you did not opt into. See [Ban user](#tool-ban-user).
