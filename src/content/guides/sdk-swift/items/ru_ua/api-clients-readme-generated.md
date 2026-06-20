The FastComments SDK предоставляет три API-клиента:

### PublicAPI - Методы, безопасные для клиентского кода

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Do not require an API key
- Can use SSO tokens for authentication
- Are rate-limited per user/device
- Are suitable for end-user facing applications

**Example use case**: Fetching and creating comments in your iOS app

### DefaultAPI - Методы для серверного кода

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Require your FastComments API key
- Should ONLY be called from server-side code
- Provide full access to your FastComments data
- Are rate-limited per tenant

**Example use case**: Administrative operations, bulk data export, user management

### ModerationAPI - Методы панели модерации

The `ModerationAPI` contains methods that power the moderator dashboard. These methods cover:
- **Comment moderation** - list, count, search, retrieve logs, and export comments
- **Moderation actions** - remove/restore comments, flag, set review/spam/approval status, manage votes, and reopen/close threads
- **Bans** - ban a user from a comment, undo bans, fetch pre-ban summaries, check ban status and preferences, and read banned-user counts
- **Badges & trust** - award/remove badges, list manual badges, get/set a user's trust factor, and read a user's internal profile

Every `ModerationAPI` method accepts an `sso` parameter so moderators can be authenticated via SSO.

**Example use case**: Building a moderation experience for moderators of your community

**ВАЖНО**: Never expose your API key in client-side code. API keys should only be used server-side.