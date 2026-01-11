FastComments Swift SDK sastoji se od nekoliko modula:

- **Client Module** - Automatski generirani API klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - I autentificirane (`DefaultAPI`) i javne (`PublicAPI`) krajnje točke
  - Potpuna podrška za async/await
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Alati za Single Sign-On na strani poslužitelja
  - Sigurna generacija tokena za autentikaciju korisnika
  - Podrška za jednostavne i sigurne SSO načine rada
  - Potpisivanje tokena temeljeno na HMAC-SHA256 koristeći CryptoKit