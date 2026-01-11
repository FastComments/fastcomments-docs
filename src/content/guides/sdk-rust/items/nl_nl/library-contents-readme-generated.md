---
De FastComments Rust SDK bestaat uit meerdere modules:

- **Client Module** - Automatisch gegenereerde API-client voor de FastComments REST-API's
  - Volledige type-definities voor alle API-modellen
  - Zowel geauthenticeerde (`DefaultApi`) als openbare (`PublicApi`) endpoints
  - Volledige async/await-ondersteuning met tokio
  - Zie [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) voor gedetailleerde API-documentatie

- **SSO Module** - Server-side Single Sign-On-hulpmiddelen
  - Veilige tokengeneratie voor gebruikersauthenticatie
  - Ondersteuning voor zowel eenvoudige als veilige SSO-modi
  - Op HMAC-SHA256 gebaseerde tokenondertekening

- **Core Types** - Gedeelde type-definities en hulpmiddelen
  - Commentaarmodellen en metadata-structuren
  - Gebruikers- en tenantconfiguraties
  - Hulpfuncties voor veelvoorkomende bewerkingen
---