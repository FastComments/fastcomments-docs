---
FastComments Rust SDK sastoji se od nekoliko modula:

- **Client Module** - Automatski generirani API klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - I autentificirane (`DefaultApi`) i javne (`PublicApi`) krajnje točke
  - Potpuna podrška async/await s tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Pomoćni alati za Single Sign-On na strani poslužitelja
  - Sigurna generacija tokena za autentikaciju korisnika
  - Podrška za jednostavne i sigurne SSO režime
  - Potpisivanje tokena na temelju HMAC-SHA256

- **Core Types** - Zajedničke definicije tipova i pomoćni alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i tenanta
  - Pomoćne funkcije za uobičajene operacije
---