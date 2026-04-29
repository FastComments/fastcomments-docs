**Template ID:** `tos_enforcer`

The Moderator template is the recommended starting point if your goal is reducing manual moderation load. It reviews new and flagged comments and applies your community rules.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt predloška moderatora'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

You will almost always want to **augment this prompt** with concrete examples of what your site does and does not allow. The platform's own escalation policy (warn before ban, search memory before banning) is already baked into the system prompt the agent receives, so you do not need to repeat it.

### Okidači

- **New comment posted** (`COMMENT_ADD`) - the agent looks at every new comment.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - the agent re-evaluates a comment that other users have flagged.

### Dozvoljeni alati

- [`mark_comment_approved`](#tools-overview) - useful for pre-moderation tenants where the agent releases clean comments and hides the rest.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

It cannot post comments, vote, pin, lock, award badges, or send email - the prompt is intentionally narrow.

### Preporučeni dodaci prije objave

- **Set [Community Guidelines](#community-guidelines).** A few sentences of written policy is enough; the agent applies it on every run.
- **Gate `ban_user` behind [approval](#approval-workflow).** This is on by default in the EU region (see [EU DSA Article 17 Compliance](#eu-dsa-compliance)) and recommended everywhere.
- **Consider also gating `mark_comment_spam` behind approval** if you have low-volume but high-stakes content.
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** Approving a bad comment puts it in front of readers; gate it until the agent has earned trust through dry-run.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** in [Context Options](#context-options). The model will warn far less aggressively when it can see that someone is a long-time good-faith user.

### Preporučeni prozor za dry-run

Run this template in [dry-run](#dry-run-mode) for at least a week against your real traffic before flipping to Enabled. Use [Test Runs (Replays)](#test-runs-replays) to also preview against the previous 30 days.

---