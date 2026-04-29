Agent memory is a tenant-scoped, **shared** key-value pool that every agent in your tenant can read from and write to. It exists so agents can carry context across runs.

### Why memory exists

LLM context is per-run. Without memory, an agent that issues a warning to a user has no way to know about that warning the next time it sees the same user. The platform's escalation policy - "warn before banning" - depends on the agent being able to find the prior warning. Memory is what makes that work.

### Two kinds of memory

- **WARNING** - written automatically as part of the [`warn_user`](#tool-warn-user) flow. The agent does not write `WARNING` records by hand; they are a side effect of warning a user.
- **NOTE** - written by [`save_memory`](#tools-overview). General-purpose context the agent wants future agents to know.

The escalation policy looks specifically for `WARNING` records when deciding whether a ban is justified.

### Tenant-scoped, agent-shared

All agents in your tenant share **one memory pool**. A note saved by Agent A is visible to Agent B's `search_memory` calls. This is intentional - you want a triage agent's notes to inform a moderator agent's decisions.

`tenantId` is set by the executor from the agent's own tenant - never from LLM args - so cross-tenant memory leaks are impossible by construction.

### What's in a memory record

Each memory entry contains:

- **Which agent wrote it**, and when.
- **Who it's about** - the user this memory describes. The agent cannot fabricate this; the platform fills it in automatically from whatever triggered the agent.
- **A hidden alt-account signal** - the platform also records (privately) the IP fingerprint of the originating comment, so future memory searches can surface notes about other accounts posting from the same IP. The fingerprint is never shown to the agent or the LLM.
- **The note itself** - up to 2000 characters of free text.
- **Tags** for retrieval - up to 10 short tags.
- **A kind** - either a warning or a general note.
- **An optional comment link** - if the memory is tied to a specific comment.

### Search behavior

[`search_memory`](#tools-overview) returns up to 25 records, sorted newest-first, scoped automatically to (the trigger's user) OR (other accounts on the trigger's IP). The results are also char-capped at 8000 total characters across all returned content - older entries are dropped if the cap is hit.

The agent does not pass `userId` or `targetIpHash`. Both are set by the executor.

### Persistence

Memory has **no TTL**. Records persist until explicitly removed. WARNING records about a user are intentionally never auto-deleted - the escalation history must be findable indefinitely or the platform's "search before banning" check is meaningless.

The three ways memory is removed:

- A moderator deletes the underlying comment - any memory tied to that comment is cascaded.
- A user is deleted - all memory entries about that user are removed in the same transaction.
- Your tenant is deleted.

There is no admin UI for deleting individual memory records today.

### Memory in dry-run

Dry-run agents do **not** write memory. This is by design: a dry-run agent's hypothetical decisions should not pollute the shared memory pool. Read-back via `search_memory` works in dry-run normally - the agent can see real memories from live agents - it just cannot add to them.

### Memory in replays

Same as dry-run: replay agents do not write memory. Replays are preview-only. See [Test Runs (Replays)](#test-runs-replays).

### Constraints summary

| 制限 | 値 |
|---|---|
| Memory content max length | 2000 chars |
| Memory tag max length | 64 chars |
| Memory tags max count | 10 |
| Memory query max length | 200 chars |
| Memory search result limit | 25 records |
| Memory search total content cap | 8000 chars |

### See also

- [ツール: save_memory](#tools-overview) for writing.
- [ツール: search_memory](#tools-overview) for reading.
- [ツール: warn_user](#tool-warn-user) - the only tool that writes WARNING-kind memory.
- [ツール: ban_user](#tool-ban-user) - the system prompt requires `search_memory` to be called before this.