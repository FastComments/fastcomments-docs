FastComments Rust SDK sastoji se od nekoliko modula:

- **Client Module** - API klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - Tri API klijenta koji pokrivaju sve FastComments metode:
    - `default_api` (**DefaultApi**) - metode autentificirane API ključem za upotrebu na strani poslužitelja
    - `public_api` (**PublicApi**) - javne metode, bez API ključa, koje je sigurno pozivati iz preglednika i mobilnih aplikacija
    - `moderation_api` (**ModerationApi**) - opsežan skup live i bržih API-ja za moderiranje. Svaka metoda Moderation prihvaća `sso` parametar i može se autentificirati putem SSO ili FastComments.com sesijskog kolačića.
  - Potpuna podrška za async/await s tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Alati za jedinstvenu prijavu (Single Sign-On) na strani poslužitelja
  - Sigurna generacija tokena za autentifikaciju korisnika
  - Podrška za jednostavne i sigurne SSO načine
  - Potpisivanje tokena bazirano na HMAC-SHA256

- **Core Types** - Zajedničke definicije tipova i alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i najamnika
  - Pomoćne funkcije za uobičajene operacije