The Warn tool sends a private DM warning to a user about a specific comment, and at the same time records the warning in shared [agent memory](#agent-memory-system). The two writes are atomic - the user never sees a warning that is not also on record.

### Why it exists

The platform's escalation policy is **warn first, ban only if the user reoffends**. The Warn tool is what makes that policy actionable: it gives the user a chance to correct course, and the warning record is what a future agent finds when it searches memory before considering a ban.

The tool also de-duplicates: if the agent has already issued a warning to the same user about the same comment, a second warning is a no-op. So an LLM that loops or re-fires on the same comment cannot spam the user with multiple warnings.

### What goes in the warning

A short message (capped at 1000 characters) shown to the user as a DM. Strong warnings are:

- **Specific** - "Personal attacks on named users are not allowed in this community" beats "your comment was flagged."
- **Short** - a few sentences max.
- **Actionable** - tell the user what to change. "Please edit your comment to remove the named user, or it will be removed."

You don't write the message yourself; the agent does, based on the [initial prompt](#personality-prompt) and [community guidelines](#community-guidelines). Your job is to write a prompt that produces good warnings.

### When to allow it

For any moderation-style agent. The Moderator template enables it by default.

### Approvals

Less commonly gated than [Ban user](#tool-ban-user). Worth gating during the first weeks of an agent's life so you can spot bad warnings before they go out, but most operators drop the gate once the agent is producing reliable output.

### See also

- [Ban user](#tool-ban-user) - the next step up in escalation.
- [Agent Memory System](#agent-memory-system) - where warning records live.