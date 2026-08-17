The FastComments SDK provides three API clients:

### PublicAPI - Client‑veilige methoden

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Do not require an API key
- Can use SSO tokens for authentication
- Are rate‑limited per user/device
- Are suitable for end‑user facing applications

**Voorbeeld gebruikssituatie**: Fetching and creating comments in your iOS app

### DefaultAPI - Server‑side methoden

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Require your FastComments API key
- Should ONLY be called from server-side code
- Provide full access to your FastComments data
- Are rate‑limited per tenant

**Voorbeeld gebruikssituatie**: Administrative operations, bulk data export, user management

### ModerationAPI - Moderator‑dashboard methoden

The `ModerationAPI` provides an extensive suite of live and fast moderation APIs. Every `ModerationAPI` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.

**Voorbeeld gebruikssituatie**: Building a moderation experience for moderators of your community

**BELANGRIJK**: Never expose your API key in client-side code. API keys should only be used server-side.