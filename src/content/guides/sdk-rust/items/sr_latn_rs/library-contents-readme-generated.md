FastComments Rust SDK se sastoji iz nekoliko modula:

- **Klijentski modul** - API klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - Tri API klijenta koja pokrivaju sve FastComments metode:
    - `default_api` (**DefaultApi**) - metode autentifikovane API ključem za upotrebu na serverskoj strani
    - `public_api` (**PublicApi**) - javne, bez API-ključa, bezbedne metode za pozivanje iz pregledača i mobilnih aplikacija
    - `moderation_api` (**ModerationApi**) - metode koje stoje iza moderator table (dashboard), uključujući moderaciju komentara (lista, broj, pretraga, zapisi, izvoz), moderacione akcije (ukloni/vrati, prijavi, postavi status za pregled/spam/odobrenje, glasovi, ponovo otvori/zatvori nit), zabrane (zabrana na osnovu komentara, poništi, predrasprave zabrana, status/preferencije zabrane, brojevi zabranjenih korisnika), i značke & poverenje (dodeli/ukloni značke, manuelne značke, dohvati/postavi faktor poverenja, interni profil korisnika). Svaka Moderation metoda prihvata `sso` parametar tako da poziv može biti napravljen u ime moderatora autentifikovanog putem SSO.
  - Potpuna podrška za async/await uz tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu API dokumentaciju

- **SSO modul** - Serverske utilitije za Single Sign-On
  - Sigurna generacija tokena za autentifikaciju korisnika
  - Podrška za jednostavan i siguran SSO režim
  - Potpisivanje tokena zasnovano na HMAC-SHA256

- **Osnovni tipovi** - Zajedničke definicije tipova i pomoćni alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i tenant-a
  - Pomoćne funkcije za uobičajene operacije