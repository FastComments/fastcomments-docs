זיכרון סוכן הוא מאגר מפתח-ערך משותף בטווח הטננט שכל סוכן בטננט שלך יכול לקרוא ולכתוב אליו. הוא קיים כדי שסוכנים יוכלו לשמר הקשר בין ריצות.

### Why memory exists

ההקשר של ה-LLM הוא לכל ריצה. בלי זיכרון, סוכן שמנפיק אזהרה למשתמש לא יוכל לדעת על אותה אזהרה בפעם הבאה שהוא יפגוש את אותו משתמש. מדיניות ההדרגה של הפלטפורמה - "להזהיר לפני חסימה" - תלויה בכך שהסוכן יוכל למצוא את האזהרה הקודמת. הזיכרון הוא מה שמאפשר זאת.

### Two kinds of memory

- **WARNING** - נכתב באופן אוטומטי כחלק מזרימת [`warn_user`](#tool-warn-user). הסוכן אינו כותב `WARNING` רשומות באופן ידני; הן תופעת לוואי של מתן אזהרה למשתמש.
- **NOTE** - נכתב על ידי [`save_memory`](#tools-overview). הקשר כללי שהסוכן רוצה שסוכנים עתידיים ידעו.

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

| Limit | Value |
|---|---|
| אורך מקסימלי של תוכן זיכרון | 2000 תווים |
| אורך מקסימלי של תג זיכרון | 64 תווים |
| מקסימום מספר תגי זיכרון | 10 |
| אורך מקסימלי של שאילתת זיכרון | 200 תווים |
| מגבלת תוצאות חיפוש זיכרון | 25 רשומות |
| מגבלת תוכן כוללת לתוצאות חיפוש זיכרון | 8000 תווים |

### See also

- [Tool: save_memory](#tools-overview) for writing.
- [Tool: search_memory](#tools-overview) for reading.
- [Tool: warn_user](#tool-warn-user) - the only tool that writes WARNING-kind memory.
- [Tool: ban_user](#tool-ban-user) - the system prompt requires `search_memory` to be called before this.

---