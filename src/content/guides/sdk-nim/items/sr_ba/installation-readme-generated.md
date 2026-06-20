### Korištenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisani API klijent i SSO pomoćne alate koji olakšavaju rad sa API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje tri API modula, `api_default`, `api_public`, i `api_moderation`. `api_default` sadrži metode koje zahtijevaju vaš API ključ, a `api_public` sadrži API pozive koji se mogu praviti direktno iz browsera/mobilnog uređaja/itd. bez autentifikacije. `api_moderation` modul sadrži metode za moderatorsku kontrolnu ploču.

`api_moderation` metode obuhvataju listanje, brojanje, pretraživanje i eksportovanje komentara i njihovih logova; moderatorske akcije poput uklanjanja/obnavljanja komentara, prijavljivanja, postavljanja statusa pregleda/spama/odobrenja, podešavanja glasova, i ponovnog otvaranja/zatvaranja niti; zabrane (zabrana korisnika od komentarisanja, poništavanje zabrane, sažeci prije zabrane, status zabrane i preferencije, i broj zabranjenih korisnika); i značke & povjerenje (dodjela/uklanjanje značke, listanje ručnih znački, dobijanje/postavljanje faktora povjerenja korisnika, i dohvatanje unutrašnjeg profila korisnika). Svaka `api_moderation` metoda prihvata `sso` parametar tako da je poziv autentifikovan kao SSO moderator.