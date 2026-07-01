The FastComments Rust SDK se sastoji od nekoliko modula:

- **Client Module** - API klijent za FastComments REST API-je
  - Potpuna definicija tipova za sve API modele
  - Tri API klijenta koji pokrivaju sve FastComments metode:
    - `default_api` (**DefaultApi**) - Metode autentifikovane API ključem za upotrebu na serveru
    - `public_api` (**PublicApi**) - javne, metode bez API ključa koje su sigurne za pozivanje iz preglednika i mobilnih aplikacija
    - `moderation_api` (**ModerationApi**) - opsežan paket live i brzih moderacijskih API‑ja. Svaka Moderation metoda prima `sso` parametar i može se autentifikovati putem SSO ili FastComments.com sesijskog kolačića.
  - Potpuna podrška za async/await uz tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Server‑side Single Sign-On (SSO) alati
  - Sigurno generisanje tokena za autentifikaciju korisnika
  - Podrška za oba, jednostavna i sigurna SSO moda
  - Potpisivanje tokena bazirano na HMAC‑SHA256

- **Core Types** - Zajedničke definicije tipova i alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i najamnika
  - Pomoćne funkcije za uobičajene operacije