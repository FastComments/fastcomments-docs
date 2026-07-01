The FastComments SDK provides three API clients:

### PublicAPI - Metode sigurne za klijenta

The `PublicAPI` contains methods that are safe to call from client‑side code (iOS/macOS apps). These methods:
- Do not require an API key → Ne zahtijevaju API ključ
- Can use SSO tokens for authentication → Mogu koristiti SSO tokene za autentifikaciju
- Are rate‑limited per user/device → Ograničeni su po brzini po korisniku/uređaju
- Are suitable for end‑user facing applications → Pogodni su za aplikacije usmjerene prema krajnjim korisnicima

**Example use case**: Fetching and creating comments in your iOS app → **Primjer upotrebe**: Preuzimanje i kreiranje komentara u vašoj iOS aplikaciji

### DefaultAPI - Metode na serveru

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Require your FastComments API key → Zahtijevaju vaš FastComments API ključ
- Should ONLY be called from server‑side code → Treba ih pozivati SAMO iz server‑side koda
- Provide full access to your FastComments data → Omogućavaju puni pristup vašim FastComments podacima
- Are rate‑limited per tenant → Ograničeni su po brzini po tenantu

**Example use case**: Administrative operations, bulk data export, user management → **Primjer upotrebe**: Administrativne operacije, izvoz podataka u bulk‑u, upravljanje korisnicima

### ModerationAPI - Metode za moderacijski dashboard

The `ModerationAPI` provides an extensive suite of live and fast moderation APIs. Every `ModerationAPI` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.

`ModerationAPI` pruža opsežan set živih i brzih API‑ja za moderaciju. Svaka `ModerationAPI` metoda prihvata `sso` parametar i može se autentifikovati putem SSO ili FastComments.com sesijskog kolačića.

**Example use case**: Building a moderation experience for moderators of your community → **Primjer upotrebe**: Izgradnja iskustva moderacije za moderatore vaše zajednice

**IMPORTANT**: Never expose your API key in client‑side code. API keys should only be used server‑side. → **VAŽNO**: Nikada ne izlažite vaš API ključ u kodu na klijentskoj strani. API ključeve treba koristiti samo na serveru.