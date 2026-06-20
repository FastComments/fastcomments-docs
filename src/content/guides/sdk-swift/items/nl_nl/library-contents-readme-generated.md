---
De FastComments Swift SDK bestaat uit meerdere modules:

- **Clientmodule** - API-client voor de FastComments REST-API's
  - Volledige typedefinities voor alle API-modellen
  - Geauthenticeerde (`DefaultAPI`), openbare (`PublicAPI`) en moderatie (`ModerationAPI`) methoden
  - Volledige async/await-ondersteuning
  - Zie [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) voor gedetailleerde API-documentatie

- **SSO-module** - Server-side Single Sign-On-hulpmiddelen
  - Veilige tokengeneratie voor gebruikersauthenticatie
  - Ondersteuning voor zowel eenvoudige als beveiligde SSO-modi
  - Op HMAC-SHA256 gebaseerde ondertekening van tokens met CryptoKit
---