By default, an agent runs across your whole tenant - every page, every locale. The **Scope** and **Locales** sections on the edit form let you narrow that.

### Restrict to specific pages

The **Restrict to specific pages** field accepts one URL pattern per line, in url-pattern glob syntax. The agent only runs on comments whose page URL matches at least one of the patterns. Examples:

- `/news/*` - any page under `/news`.
- `/forums/*` - any page under `/forums`.
- `/blog/2026/*` - any page under `/blog/2026`.
- (multiple lines together) - the agent runs if **any** pattern matches.

Maximum: 50 patterns per agent. Patterns must be valid url-pattern globs - the form rejects malformed ones with a specific error.

When the field is **blank**, the agent runs on every page in the tenant.

When the field is **non-blank**, the agent fails closed: any trigger whose comment has no `urlId` (e.g. tenant-level events with no page context) is skipped. This is by design - "scoped to /news/*" should not silently fall through to "everything".

### Restrict to specific locales

The **Restrict to specific locales** dual-list picker accepts FastComments locale IDs (`en_us`, `zh_cn`, `de_de`, etc.). The agent only runs on comments whose detected locale is in the selected list.

Detected locale comes from the comment's `locale` field, which is set by the comment widget at post time based on the page locale.

When **no locales** are selected, the agent runs on every locale.

When **one or more locales** are selected, the agent fails closed: triggers without a comment, or triggers on comments with no `locale` field, are skipped.

### Combined scoping

URL and locale filters AND together. A trigger only fires the agent if **both** filters allow it.

Useful patterns:
- `/news/*` URL pattern + `en_us` locale - English news section only.
- No URL filter + multiple locales - tenant-wide, but only for the languages this agent's prompt was written for.

### Why scope an agent

- **Cost.** Scoping cuts the volume of triggers the agent has to process, and so cuts token spend.
- **Correctness.** A summarizer prompt tuned for technical articles may produce poor output on product pages. Scoping is a sharper tool than asking the prompt to "skip non-technical pages" in English.
- **Locale-specific behavior.** A welcome greeter that only writes in German should only run on German-locale comments. Combine `de_de` locale scope with a German-language tone in the [initial prompt](#personality-prompt).

### What scoping does *not* do

- It does not change the **agent slot count** (see [Plans and Eligibility](#plans-and-eligibility)) - a scoped agent still occupies one slot.
- It does not change [Budget caps](#budgets-overview) - the per-agent daily and monthly caps apply across all matching triggers.
- It does not retroactively scope past runs - run history shows everything the agent did, even if you scope it tighter afterwards.
