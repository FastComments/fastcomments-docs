The FastComments SDK provides three API clients:

### PublicAPI - Klientsikre metoder

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Do not require an API key → Kræver ikke en API-nøgle
- Can use SSO tokens for authentication → Kan bruge SSO‑tokens til godkendelse
- Are rate-limited per user/device → Er hastighedsbegrænset pr. bruger/enhed
- Are suitable for end-user facing applications → Er egnet til slutbruger‑orienterede applikationer

**Eksempel på brug**: Fetching and creating comments in your iOS app → Hentning og oprettelse af kommentarer i din iOS‑app

### DefaultAPI - Server‑side metoder

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Require your FastComments API key → Kræver din FastComments API‑nøgle
- Should ONLY be called from server-side code → Bør KUN kaldes fra server‑side kode
- Provide full access to your FastComments data → Giver fuld adgang til dine FastComments‑data
- Are rate-limited per tenant → Er hastighedsbegrænset pr. lejer

**Eksempel på brug**: Administrative operations, bulk data export, user management → Administrative operationer, masse‑dataeksport, brugeradministration

### ModerationAPI - Moderatorsdashboard‑metoder

The `ModerationAPI` provides an extensive suite of live and fast moderation APIs. Every `ModerationAPI` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie. → `ModerationAPI` leverer en omfattende suite af live og hurtige moderations‑API'er. Hver `ModerationAPI`‑metode accepterer en `sso`‑parameter og kan autentificeres via SSO eller en FastComments.com‑sessions‑cookie.

**Eksempel på brug**: Building a moderation experience for moderators of your community → Bygge en moderationsoplevelse for moderatorer af dit fællesskab

**VIGTIGT**: Never expose your API key in client-side code. API keys should only be used server-side. → Udsæt aldrig din API‑nøgle i klient‑side kode. API‑nøgler bør kun bruges på server‑siden.