The FastComments SDK pruža tri API klijenta:

### PublicAPI – Metode sigurne za klijenta

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Do not require an API key
- Can use SSO tokens for authentication
- Are rate-limited per user/device
- Are suitable for end-user facing applications

**Primjer upotrebe**: Fetching and creating comments in your iOS app

### DefaultAPI – Metode na strani servera

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Require your FastComments API key
- Should ONLY be called from server-side code
- Provide full access to your FastComments data
- Are rate-limited per tenant

**Primjer upotrebe**: Administrative operations, bulk data export, user management

### ModerationAPI – Metode nadzorne ploče moderatora

The `ModerationAPI` provides an extensive suite of live and fast moderation APIs. Every `ModerationAPI` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.

**Primjer upotrebe**: Building a moderation experience for moderators of your community

**VAŽNO**: Never expose your API key in client-side code. API keys should only be used server-side.