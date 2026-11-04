FastComments SDK'en giver tre API-klienter:

### PublicAPI - Klientsikre metoder

`PublicAPI` indeholder metoder, der er sikre at kalde fra klient-side kode (iOS/macOS-apps). Disse metoder:
- Kræver ikke en API-nøgle
- Kan bruge SSO-tokens til autentificering
- Er rate-begrænsede pr. bruger/enhed
- Er velegnede til applikationer rettet mod slutbrugere

**Eksempel på brugsscenarie**: Hentning og oprettelse af kommentarer i din iOS-app

### DefaultAPI - Server-side metoder

`DefaultAPI` indeholder autentificerede metoder, der kræver en API-nøgle. Disse metoder:
- Kræver din FastComments API-nøgle
- Skun KUN kaldes fra server-side kode
- Giver fuld adgang til dine FastComments-data
- Er rate-begrænsede pr. tenant

**Eksempel på brugsscenarie**: Administrative operationer, bulk-dataeksport, brugeradministration

### ModerationAPI - Metoder til moderator-dashboardet

`ModerationAPI` giver en omfattende suite af live- og hurtige moderations-API'er. Hver `ModerationAPI`-metode accepterer en `sso`-parameter og kan autentificeres via SSO eller en FastComments.com-sessioncookie.

**Eksempel på brugsscenarie**: Opbygning af en moderationsoplevelse for moderatorer af dit community

**VIGTIGT**: Udsend aldrig din API-nøgle i klient-side kode. API-nøgler bør kun bruges server-side.