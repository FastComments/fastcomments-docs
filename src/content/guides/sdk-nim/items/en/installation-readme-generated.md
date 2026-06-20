### Using Nimble

```bash
nimble install fastcomments
```

### Building from Source

```bash
nimble build
```

### Library Contents

This library contains the generated API client and the SSO utilities to make working with the API easier.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Public vs Secured APIs

For the API client, there are three API modules, `api_default`, `api_public`, and `api_moderation`. The `api_default` contains methods that require your API key, and `api_public` contains api calls
that can be made directly from a browser/mobile device/etc without authentication. The `api_moderation` module contains methods for the moderator dashboard.

The `api_moderation` methods cover listing, counting, searching, and exporting comments and their logs; moderation actions like removing/restoring comments, flagging, setting review/spam/approval status, adjusting votes, and reopening/closing threads; bans (banning a user from a comment, undoing a ban, pre-ban summaries, ban status and preferences, and banned-user counts); and badges & trust (awarding/removing a badge, listing manual badges, getting/setting a user's trust factor, and fetching a user's internal profile). Every `api_moderation` method accepts an `sso` parameter so the call is authenticated as an SSO moderator.