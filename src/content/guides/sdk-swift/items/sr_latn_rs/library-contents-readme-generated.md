---
The FastComments Swift SDK se sastoji od nekoliko modula:

- **Client Module** - Automatski generisan API klijent za FastComments REST API-je
  - Kompletne definicije tipova za sve API modele
  - I autentifikovani (`DefaultAPI`) i javni (`PublicAPI`) krajnje tačke
  - Potpuna podrška za async/await
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO Module** - Alatke za Single Sign-On na serverskoj strani
  - Sigurna generacija tokena za autentifikaciju korisnika
  - Podrška za oba režima SSO: jednostavan i bezbedan
  - Potpisivanje tokena zasnovano na HMAC-SHA256 koristeći CryptoKit
---