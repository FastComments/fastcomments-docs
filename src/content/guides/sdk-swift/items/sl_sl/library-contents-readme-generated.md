FastComments Swift SDK sestoji iz več modulov:

- **Client Module** - Samodejno ustvarjen API odjemalec za FastComments REST API-je
  - Celovite definicije tipov za vse modele API-ja
  - Tako avtenticirane (`DefaultAPI`) kot javne (`PublicAPI`) končne točke
  - Popolna podpora za async/await
  - Oglejte si [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) za podrobno dokumentacijo API-ja

- **SSO Module** - Strežniška orodja za Single Sign-On
  - Varno ustvarjanje žetonov za overjanje uporabnikov
  - Podpora tako za preproste kot za varne načine SSO
  - Podpisovanje žetonov na osnovi HMAC-SHA256 z uporabo CryptoKit