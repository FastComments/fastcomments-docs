The Ban tool is the most consequential action an agent can call. It bans a user from your community, with a fixed duration and a few options.

### What it does

The agent picks one of six durations:

- One hour
- One day
- One week
- One month
- Six months
- One year

It also picks between a **visible ban** (the user sees a clear ban message and can appeal) and a **shadow ban** (the user can keep posting but their content is hidden from other users). The platform's instructions tell the agent to prefer visible bans for first-time or borderline cases, and shadow bans for clearly malicious repeat offenders.

### The two destructive sub-options

Two extra options are **hidden from the agent by default**. To enable either, tick the corresponding checkbox in the **Ban options** section on the agent's edit form:

- **Allow deleting all of the user's comments.** When enabled, the agent can choose to also delete every comment the banned user has ever posted in your tenant. Reserve for clear spam, doxxing, or coordinated abuse where the existing content has no value. **Destructive and irreversible.**
- **Allow banning by IP.** When enabled, the agent can choose to also ban the IP the comment was posted from. Useful against alt-account ban evasion. **Avoid for shared IPs** (corporate, school, mobile carriers) - innocent users on the same network will be blocked.

The platform also clamps these server-side: even if the agent goes rogue and tries to invoke the option, the request is refused unless you opted in.

### Escalation policy

Before banning, the platform instructs the agent to:

1. Search [agent memory](#agent-memory-system) for prior warnings or notes about the user.
2. Prefer [warning](#tool-warn-user) the user over banning for first offenses.
3. Only skip the warning step for clearly egregious cases (illegal content, doxxing, coordinated spam) - and explain why in its justification.

This policy is in the agent's instructions, not a hard server-side rule, which is why **gating bans behind approval is strongly recommended**.

### EU region: human approval required

In the EU region, this tool is **locked on for approval** by Article 17 of the Digital Services Act. Every ban from any agent on an EU-region tenant lands in the [approvals inbox](#approval-workflow) for human review. See [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Recommendations

- Gate behind approval everywhere for at least the first month.
- Always gate the **delete-all-comments** option if you enable it - it is irreversible.
- Consider gating the **IP ban** option even after the agent earns trust - the cost of an IP ban on a shared network does not show up in the agent's run history.

### See also

- [Banning Users](/guide-moderation.html#banning-users) and [Banning Users With Wildcards](/guide-moderation.html#banning-users-wildcards) in the moderation guide for how bans work platform-wide.
- [Warn user](#tool-warn-user) - the gentler escalation step.
