De FastComments Swift SDK bestaat uit verschillende modules:

- **Client Module** - Automatisch gegenereerde API-client voor de FastComments REST-API's
  - Volledige typedefinities voor alle API-modellen
  - Zowel geauthenticeerde (`DefaultAPI`) als openbare (`PublicAPI`) endpoints
  - Volledige async/await-ondersteuning
  - Zie [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) voor gedetailleerde API-documentatie

- **SSO Module** - Server-side Single Sign-On-hulpmiddelen
  - Beveiligde tokengeneratie voor gebruikersauthenticatie
  - Ondersteuning voor zowel eenvoudige als beveiligde SSO-modi
  - Op HMAC-SHA256 gebaseerde tokenondertekening met CryptoKit