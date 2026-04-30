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

Lets the agent post a comment as itself. The comment is shown publicly under the agent's display name. Used by greeter and summarizer agents. Reversible - any moderator can remove a bad comment. Gate behind approval if your community needs every public-facing message to be human-reviewed.

#### Editing a comment

Lets the agent rewrite the text of an in-scope comment. The original text is preserved in the comment's audit log. Reserve for narrow cases - redacting PII a user leaked, or amending the agent's own prior reply. Not for rewriting opinions or softening tone. See [Edit comment](#tool-edit-comment) for the full page.

#### Voting on comments

Lets the agent vote up or down on a comment. The vote counts toward the comment's vote total like any other vote. Most communities prefer not to have bots voting; not enabled in any starter template. If you do allow it, voting is reversible.

#### Pin / unpin a comment

Lets the agent pin a comment to the top of the page or unpin one that is already pinned. The platform does not enforce a one-pin-per-thread rule, so a pinning agent should be instructed to unpin the previous pinned comment first. To discover what is already pinned on the same page, the agent can call the read-only `get_pinned_comments` tool (see below). Used by the Top Comment Pinner template.

#### Lock / unlock a comment

Lets the agent prevent further replies under a comment, or restore replies. The locked comment stays visible. Useful for cool-downs on heated threads, paired with a deferred unlock. To discover what is currently locked on the same page, the agent can call the read-only `get_locked_comments` tool (see below).

#### Mark / unmark spam

Lets the agent mark a comment as spam (hiding it from readers and feeding the spam classifier) or clear that flag. The bread-and-butter tool for any moderation agent. Reversible.

#### Approve / un-approve a comment

Lets the agent show a held comment to readers, or hide an already-visible one. Most useful on tenants that hold new comments for moderator review.

#### Mark a comment reviewed

A queue-state tool: marks a comment as "a moderator (or agent) has looked at this." Does not change visibility. Low stakes; rarely gated.

#### Award a badge

Lets the agent give a user a badge you have configured for your tenant. Reversible by a moderator. When this tool is enabled, the agent can see your tenant's badges and pick the right one on its own, so you do not need to paste badge identifiers into your community guidelines or initial prompt. To steer which badge gets awarded for what behavior, refer to the badges by their **Display Label** in the prompt.

#### Send email

Lets the agent send a plain-text email to the author of a comment in the trigger's scope. The agent never sees the recipient's email address - it picks a comment and the platform delivers to whatever address that commenter left when they posted. The from-address is your tenant's branded sender (with DKIM) when the comment's domain matches a configured domain, otherwise the platform default. Use sparingly - email is the highest-friction tool and bad emails are hard to undo.

#### Save / search agent memory

Two paired tools that read and write a shared notes pool about the user a trigger fired for. Memory is shared across all agents in your tenant, so a triage agent's notes inform a moderator agent's decisions. Search is read-only and always available; saving is rarely gated. See [Agent Memory System](#agent-memory-system) for the full design.

#### Get pinned comments / Get locked comments

Two read-only discovery tools that list the pinned (or locked) comments on the same page (`urlId`) the trigger fired on. They take no arguments - the page is read from the trigger context, so the agent cannot pivot to other pages. Use them when an agent needs to act on a comment that is already pinned or locked - typically the first call before `unpin_comment` or `unlock_comment`, or before pinning a new comment so the existing one can be unpinned first.

Each tool is gated separately in **Allowed tool calls** (the admin ticks `List pinned comments on the current page` or `List locked comments on the current page`). They cannot be gated behind approval - read-only tools have no side effect to approve. Calling them is not recorded as an action in run history; only the resulting `unpin_comment` / `unlock_comment` / `pin_comment` call (if any) shows up. The list is capped at the most recent 20 matches per call.

Important to understand: when one of these tools returns a commentId, that commentId is added to the agent's per-run scope, so the follow-up `unpin_comment` / `unlock_comment` call validates against the platform's tool-target safety check. Without first calling the discovery tool, the agent cannot act on comments that are not already in the trigger's immediate scope. So an unpin-style agent typically gets both tools enabled (e.g. `get_pinned_comments` plus `unpin_comment`).

#### Warn a user

Sends a private DM warning to a user about a specific comment, and atomically records the warning in agent memory. The platform's escalation policy is built around this tool - warn first, ban only if the user reoffends. See [Warn user](#tool-warn-user) for the full page.

#### Ban a user

The most consequential tool an agent can call. Bans a user with a fixed duration, optionally as a shadow ban, optionally also banning the IP, optionally also deleting all of the user's comments. The two destructive options (IP, delete-all) are gated behind extra opt-ins on the edit form. In the EU region, all bans require human approval (see [EU DSA Article 17 Compliance](#eu-dsa-compliance)). See [Ban user](#tool-ban-user) for the full page.

### Ban-tool sub-options

The Ban tool exposes two destructive options - delete-all-comments and ban-by-IP - that are hidden from the model entirely until you opt them in via the **Ban options** section on the edit form. Even if the model hallucinates the parameter, the platform refuses values you did not opt into. See [Ban user](#tool-ban-user).
