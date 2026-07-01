The FastComments Rust SDK bestaat uit verschillende modules:

- **Client Module** - API-client voor FastComments REST API's
  - Volledige type-definities voor alle API-modellen
  - Drie API-clients die alle FastComments-methoden dekken:
    - `default_api` (**DefaultApi**) - methoden geauthenticeerd met API-sleutel voor server-side gebruik
    - `public_api` (**PublicApi**) - openbare, zonder API-sleutel methoden die veilig kunnen worden aangeroepen vanuit browsers en mobiele apps
    - `moderation_api` (**ModerationApi**) - een uitgebreide suite van live en snelle moderatie-API's. Elke Moderatie-methode neemt een `sso`-parameter aan en kan authenticeren via SSO of een FastComments.com sessiecookie.
  - Volledige async/await-ondersteuning met tokio
  - Zie [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) voor gedetailleerde API-documentatie

- **SSO Module** - Server-side Single Sign-On hulpprogramma's
  - Veilige tokengeneratie voor gebruikersauthenticatie
  - Ondersteuning voor zowel eenvoudige als beveiligde SSO-modus
  - HMAC-SHA256 gebaseerde tokenondertekening

- **Core Types** - Gedeelde type-definities en hulpprogramma's
  - Commentaarmodellen en metadata-structuren
  - Gebruiker- en tenantconfiguraties
  - Helper-functies voor veelvoorkomende bewerkingen