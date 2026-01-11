---
FastComments SDK'en leverer to typer API-endepunkter:

### PublicAPI - Endepunkter sikre for klienter

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- Kræver ikke en API-nøgle
- Kan bruge SSO-tokens til autentificering
- Er ratebegrænsede pr. bruger/enhed
- Er velegnede til applikationer rettet mod slutbrugere

**Eksempel på anvendelse**: Hentning og oprettelse af kommentarer i din iOS-app

### DefaultAPI - Endepunkter på serversiden

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- Kræver din FastComments API-nøgle
- Bør KUN kaldes fra server-side kode
- Giver fuld adgang til dine FastComments-data
- Er ratebegrænsede pr. tenant

**Eksempel på anvendelse**: Administrative operationer, eksport af store datamængder, moderationsværktøjer

**VIGTIGT**: Udsæt aldrig din API-nøgle i klient-side kode. API-nøgler bør kun bruges på serversiden.
---