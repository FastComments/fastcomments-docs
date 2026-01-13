FastComments Rust SDK je sestavljen iz več modulov:

- **Client Module** - Samodejno ustvarjen odjemalec API za FastComments REST API-je
  - Celovite definicije tipov za vse modele API-ja
  - Tako avtenticirane (`DefaultApi`) kot javne (`PublicApi`) končne točke
  - Popolna podpora async/await z `tokio`
  - Oglejte si [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za podrobno dokumentacijo API-ja

- **SSO Module** - Strežniška orodja za enotno prijavo (Single Sign-On)
  - Varno ustvarjanje žetonov za preverjanje pristnosti uporabnika
  - Podpora tako preprostim kot varnim načinom SSO
  - Podpisovanje žetonov na osnovi HMAC-SHA256

- **Core Types** - Skupne definicije tipov in pripomočki
  - Modeli komentarjev in strukture metapodatkov
  - Konfiguracije uporabnikov in najemnikov
  - Pomožne funkcije za pogoste operacije