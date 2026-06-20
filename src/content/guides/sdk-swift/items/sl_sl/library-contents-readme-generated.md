---
FastComments Swift SDK je sestavljen iz več modulov:

- **Client Module** - API odjemalec za FastComments REST API-je
  - Celovite definicije tipov za vse API modele
  - Metode: avtenticirane (`DefaultAPI`), javne (`PublicAPI`) in za moderacijo (`ModerationAPI`)
  - Popolna podpora za async/await
  - Oglejte si [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) za podrobno dokumentacijo API-ja

- **SSO Module** - Strežniški pripomočki za Single Sign-On
  - Varen postopek generiranja žetonov za preverjanje pristnosti uporabnika
  - Podpora tako za preprost kot varen način SSO
  - Podpisovanje žetonov, temelječe na HMAC-SHA256, z uporabo CryptoKit
---