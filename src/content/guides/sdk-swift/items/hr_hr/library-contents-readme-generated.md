FastComments Swift SDK se sastoji od nekoliko modula:

- **Client Module** - Klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - Autentificirane (`DefaultAPI`), javne (`PublicAPI`) i moderacijske (`ModerationAPI`) metode
  - Potpuna podrška za async/await
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) za detaljnu dokumentaciju API-ja

- **SSO Module** - Alati za Single Sign-On na strani poslužitelja
  - Sigurna generacija tokena za autentikaciju korisnika
  - Podrška za jednostavne i sigurne SSO načine
  - Potpisivanje tokena temeljeno na HMAC-SHA256 pomoću CryptoKit