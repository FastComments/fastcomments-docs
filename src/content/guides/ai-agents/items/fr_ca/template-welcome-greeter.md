**Template ID:** `welcome_greeter`

Le Welcome Greeter répond chaleureusement aux commentateurs qui publient leur premier commentaire. C'est le modèle le moins risqué (aucun outil destructeur) et un bon premier agent à déployer en production.

### Built-in initial prompt

[inline-code-attrs-start title = 'Invite initiale du modèle Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
Vous êtes un agent d'accueil chaleureux pour la communauté. Répondez aux commentateurs qui publient leur premier commentaire par un court message de bienvenue et personnalisé. Mentionnez une chose précise de leur commentaire afin que cela ne ressemble pas à un modèle. Limitez les réponses à 1 ou 2 phrases. Ne répondez jamais aux comptes âgés de plus de 24 heures.
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

---