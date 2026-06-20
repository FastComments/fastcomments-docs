### PyPI

```bash
pip install fastcomments
```

### Library Contents

This library contains two modules: the generated API client and the core Python library which contains hand-written utilities to make working with the API easier, including SSO support.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains methods that can be made directly from a browser/mobile device/etc without authentication. The `ModerationApi` powers the moderator dashboard and contains methods for moderating comments (list, count, search, logs, export), moderation actions (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), bans (ban from comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), and badges & trust (award/remove badge, manual badges, get/set trust factor, user internal profile). Every `ModerationApi` method accepts an `sso` parameter so it can be called on behalf of an SSO-authenticated moderator.