The FastComments Swift SDK se sastoji od nekoliko modula:

- **Klijentski modul** - API klijent za FastComments REST APIs
  - Kompletne definicije tipova za sve API modele
  - Autentifikovane (`DefaultAPI`), javne (`PublicAPI`), i moderacione (`ModerationAPI`) metode
  - Potpuna podrška za async/await
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO modul** - Alati za Single Sign-On na serverskoj strani
  - Sigurna generacija tokena za autentifikaciju korisnika
  - Podrška za jednostavne i sigurne SSO režime
  - Potpisivanje tokena zasnovano na HMAC-SHA256 koristeći CryptoKit