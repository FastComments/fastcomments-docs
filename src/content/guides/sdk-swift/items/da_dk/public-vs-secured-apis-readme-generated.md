FastComments SDK'en giver to typer API-endpoints:

### PublicAPI - Klientsikre Endpoints

`PublicAPI` indeholder endpoints, der er sikre at kalde fra klient-side kode (iOS/macOS-apps). Disse endpoints:
- Kræver ikke en API-nøgle
- Kan bruge SSO-tokens til autentificering
- Er rate-begrænsede pr. bruger/enhed
- Er velegnede til applikationer rettet mod slutbrugere

**Eksempel på brugsscenarie**: Hentning og oprettelse af kommentarer i din iOS-app

### DefaultAPI - Server-side Endpoints

`DefaultAPI` indeholder autentificerede endpoints, der kræver en API-nøgle. Disse endpoints:
- Kræver din FastComments API-nøgle
- Skun KUN kaldes fra server-side kode
- Giver fuld adgang til dine FastComments-data
- Er rate-begrænsede pr. tenant

**Eksempel på brugsscenarie**: Administrative operationer, bulk-eksport af data, moderationsværktøjer

**VIGTIGT**: Udsend aldrig din API-nøgle i klient-side kode. API-nøgler må kun bruges server-side.