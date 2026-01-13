The FastComments Rust SDK se sastoji od nekoliko modula:

- **Client Module** - Automatski generisan API klijent za FastComments REST API-je
  - Kompletne definicije tipova za sve API modele
  - I autentifikovani (`DefaultApi`) i javni (`PublicApi`) krajnji tačke
  - Potpuna podrška za async/await uz tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Alati za Single Sign-On (SSO) na serverskoj strani
  - Sigurno generisanje tokena za autentifikaciju korisnika
  - Podrška za jednostavne i sigurne SSO režime
  - Potpisivanje tokena zasnovano na HMAC-SHA256

- **Core Types** - Zajedničke definicije tipova i alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i tenant-a
  - Pomoćne funkcije za uobičajene operacije