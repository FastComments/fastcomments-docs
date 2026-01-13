FastComments Swift SDK består af flere moduler:

- **Client Module** - Autogenereret API-klient til FastComments REST API'er
  - Komplette typedefinitioner for alle API-modeller
  - Både autentificerede (`DefaultAPI`) og offentlige (`PublicAPI`) endepunkter
  - Fuld async/await-understøttelse
  - Se [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) for detaljeret API-dokumentation

- **SSO Module** - Server-side Single Sign-On-værktøjer
  - Sikker token-generering til brugerautentificering
  - Understøtter både simple og sikre SSO-tilstande
  - HMAC-SHA256-baseret tokensignering ved hjælp af CryptoKit