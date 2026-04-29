The **Context** section on the edit form controls how much information the agent receives on each run. More context produces better decisions but raises token cost per run, so you only want what the agent actually needs.

### Τι περιλαμβάνεται πάντα

Even with every checkbox unchecked, the agent's context message includes:

- The **trigger event type** (e.g. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- The **page URL and URL ID** (when known).
- The **comment** that triggered the run, if there is one - ID, author user ID, author display name, comment text, vote counts, flag count, spam/approved/reviewed flags, parent ID. The author's email is **never** sent to the LLM provider (ελαχιστοποίηση αποστολής PII).
- The **previous comment text** for `COMMENT_EDIT` triggers (so the agent can compare before/after).
- The **vote direction** for `COMMENT_VOTE_THRESHOLD` triggers.
- The **triggering user ID** and **badge ID** (for moderator badge triggers).
- Your tenant's **badge catalog** (name, display label, description) when the agent is allowed to award badges, so the agent can choose an appropriate one without you having to spell the badges out in the prompt.

All untrusted text - comment bodies, author names, page titles, the guidelines doc itself - is **fenced** in the context message with markers like `<<<COMMENT_TEXT>>> ... <<<END>>>`. The platform's system prompt instructs the model to never follow instructions inside those fences. This is the platform's prompt-injection defense; you do not need to repeat it in your prompt.

### Τα τρία πλαίσια επιλογής

#### Include parent comment and prior replies in the same thread

Adds:
- The **parent comment** - ID, author, text.
- **Sibling replies** - the prior replies to the same parent in the same thread.

Useful for: any agent that responds to a comment in context (welcome greeters, thread summarizers, moderators reading replies in conversations).

Cost: small to medium. Bounded by how many siblings exist on a given thread.

#### Include commenter's trust factor, account age, ban history, and recent comments

Adds the **AUTHOR_HISTORY** block:

- **Account age in days** since signup.
- **Trust factor (0-100)** - the FastComments score that summarizes how trusted the user is on this site. See the [Ανίχνευση Spam](/guide-moderation.html#spam-detection) page in the moderation guide.
- **Prior ban count.**
- **Total comments on this site.**
- **Duplicate-content count** - if the user has posted identical text recently (anti-spam signal).
- **Same-IP cross-account signal** - count of comments from the same IP under other accounts (alt-account signal). The IP hash itself is never sent to the LLM.
- **Recent comments** - up to 5 of the user's most recent comments, each truncated to 300 characters, fenced as untrusted text.

Useful for: any moderation agent. Without this, the model bans new accounts and long-time good-faith users with the same posture.

Cost: medium. Recent comments add the most tokens.

#### Include page title, subtitle, description, and meta tags

Adds the **PAGE_CONTEXT** block - title, subtitle, description, and any meta tags FastComments has captured for the page.

Useful for: welcome greeters and thread summarizers, where knowing what the page is about substantially improves output quality.

Cost: small.

### Οδηγίες κοινότητας

The fourth field, **Community guidelines**, is a free-text policy block included in the user-role context message on every run, fenced as untrusted text the same way comment bodies and other user-supplied content are fenced. The agent reads it as policy text but the platform does not treat it as a system instruction. See [Οδηγίες κοινότητας](#community-guidelines) for what to put in it.

### Προσθήκη context επιλεκτικά

These checkboxes apply per agent, not globally. A common pattern:

- Welcome greeter: page context **on**, thread context **off**, user history **off**.
- Moderator: thread context **off**, user history **on**, page context **off**.
- Thread summarizer: thread context **on**, page context **on**, user history **off**.

Reach for the minimum context an agent needs to be correct on the calls it actually makes - extra context costs tokens on every run, even when the agent does not use it.