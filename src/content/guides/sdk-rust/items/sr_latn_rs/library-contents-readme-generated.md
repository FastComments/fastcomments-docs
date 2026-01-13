---
The FastComments Rust SDK se sastoji od nekoliko modula:

- **Client Module** - Automatski generisan API klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - I autentifikovani (`DefaultApi`) i javni (`PublicApi`) endpointi
  - Puna podrška za async/await sa tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Alati za Single Sign-On na serverskoj strani
  - Sigurna generacija tokena za autentifikaciju korisnika
  - Podrška za i jednostavan i siguran SSO režim
  - Potpisivanje tokena zasnovano na HMAC-SHA256

- **Core Types** - Zajedničke definicije tipova i alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i tenanta
  - Pomoćne funkcije za uobičajene operacije
---