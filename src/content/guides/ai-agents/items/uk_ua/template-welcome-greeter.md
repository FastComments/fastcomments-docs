**Template ID:** `welcome_greeter`

The Welcome Greeter replies warmly to first-time commenters. It is the lowest-risk template (no destructive tools) and a good first agent to ship live.

### Built-in initial prompt

[inline-code-attrs-start title = 'Початковий запит шаблону Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Triggers

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

This event fires exactly once per user, so the agent cannot loop. See [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Allowed tools

- [`write_comment`](#tools-overview)

That is the only tool - the agent literally cannot moderate, vote, ban, or DM.

### Recommended additions before going live

- **Set the Display name** to something inviting - "Community Bot", your site mascot, or your brand name. The display name is what readers see attached to the welcome reply.
- **Tick "Include page title, subtitle, description, and meta tags"** in [Context Options](#context-options). The greeter's replies become noticeably better when it can reference what the page is actually about.
- **Consider locale restrictions** if you operate in multiple languages. A welcome reply in the wrong language is more jarring than a missed reply. See [Scope: URL and Locale Filters](#scope-url-locale).

### Why no approvals are needed

The agent only writes new comments and only on a one-shot trigger. Worst case: an awkward greeting. There is no destructive action to gate. Most operators run this one with no approvals at all once dry-run looks clean.