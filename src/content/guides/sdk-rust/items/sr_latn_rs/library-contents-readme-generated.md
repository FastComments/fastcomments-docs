FastComments Rust SDK sastoji se od nekoliko modula:

- **Client Module** - API klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - Tri API klijenta koja pokrivaju sve FastComments metode:
    - `default_api` (**DefaultApi**) - metode autentifikovane API‑klučem za upotrebu na serveru
    - `public_api` (**PublicApi**) - javne, metode bez API‑kljuca koje su sigurne za pozivanje iz pretraživača i mobilnih aplikacija
    - `moderation_api` (**ModerationApi**) - opsežan skup live i brzih API‑ja za moderaciju. Svaka metoda Moderation prima `sso` parametar i može se autentifikovati putem SSO ili FastComments.com sesijskog kolačića.
  - Potpuna podrška za async/await uz tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Server-side Single Sign-On alati
  - Sigurno generisanje tokena za autentifikaciju korisnika
  - Podrška za oba, jednostavna i sigurna SSO režima
  - Potpisivanje tokena bazirano na HMAC‑SHA256

- **Core Types** - Zajedničke definicije tipova i alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i tenant‑a
  - Pomoćne funkcije za uobičajene operacije