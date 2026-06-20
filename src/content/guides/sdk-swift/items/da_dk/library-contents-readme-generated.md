The FastComments Swift SDK består af flere moduler:

- **Client Module** - API-klient til FastComments REST-API'er
  - Fuldstændige typedefinitioner for alle API-modeller
  - Autentificerede (`DefaultAPI`), offentlige (`PublicAPI`) og moderationsmetoder (`ModerationAPI`)
  - Fuld understøttelse af async/await
  - Se [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) for detaljeret API-dokumentation

- **SSO Module** - Server-side Single Sign-On-værktøjer
  - Sikker token-generering til brugergodkendelse
  - Understøtter både simple og sikre SSO-tilstande
  - HMAC-SHA256-baseret token-signering ved hjælp af CryptoKit