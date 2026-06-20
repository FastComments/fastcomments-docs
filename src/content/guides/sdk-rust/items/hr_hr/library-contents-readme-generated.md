FastComments Rust SDK sastoji se od nekoliko modula:

- **Client Module** - API klijent za FastComments REST API-je
  - Potpune definicije tipova za sve API modele
  - Tri API klijenta koja pokrivaju sve FastComments metode:
    - `default_api` (**DefaultApi**) - metode autentificirane API-ključem za uporabu na strani poslužitelja
    - `public_api` (**PublicApi**) - javne metode bez API ključa koje je sigurno pozivati iz preglednika i mobilnih aplikacija
    - `moderation_api` (**ModerationApi**) - metode koje stoje iza moderator nadzorne ploče, uključujući moderiranje komentara (listanje, broj, pretraživanje, zapisi, izvoz), akcije moderiranja (ukloni/vrati, označi, postavi status za pregled/spam/odobrenje, glasovi, ponovno otvaranje/zatvaranje niti), zabrane (blokiranje na temelju komentara, poništi, sažeci prije zabrane, status/preferencije zabrane, broj zabranjenih korisnika) i značke & povjerenje (dodjeljivanje/uklanjanje znački, ručne značke, dohvat/postavljanje faktora povjerenja, interni korisnički profil). Svaka Moderation metoda prihvaća parametar `sso` kako bi se poziv mogao izvršiti u ime moderatora autentificiranog putem SSO-a.
  - Potpuna podrška za async/await uz tokio
  - Pogledajte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za detaljnu dokumentaciju API-ja

- **SSO Module** - Alati za Single Sign-On na strani poslužitelja
  - Sigurno generiranje tokena za autentifikaciju korisnika
  - Podrška za jednostavne i sigurne SSO načine rada
  - Potpisivanje tokena temeljeno na HMAC-SHA256

- **Core Types** - Zajedničke definicije tipova i pomoćni alati
  - Modeli komentara i strukture metapodataka
  - Konfiguracije korisnika i tenant-a
  - Pomoćne funkcije za uobičajene operacije