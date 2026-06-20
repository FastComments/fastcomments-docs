### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generirani API klijent i osnovna Python biblioteka koja sadrži ručno napisane pomoćne funkcije koje olakšavaju rad s API-jem, uključujući podršku za SSO.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija jezgre biblioteke, uključujući primjere SSO-a](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni nasuprot zaštićenim API-jima

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati izravno iz preglednika/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pokreće nadzornu ploču moderatora i sadrži metode za moderiranje komentara (popis, broj, pretraživanje, zapisi, izvoz), moderacijske akcije (ukloni/vrati, prijavi, postavi status pregled/spam/odobrenje, glasovi, ponovno otvori/zatvori nit), zabrane (zabrana komentiranja, poništi, sažeci prije zabrane, status/preferencije zabrane, broj zabranjenih korisnika) te značke i povjerenje (dodijeli/ukloni značku, ručne značke, dohvati/postavi faktor povjerenja, interni profil korisnika). Svaka metoda `ModerationApi` prihvaća parametar `sso` kako bi se mogla pozvati u ime moderatora autentificiranog putem SSO-a.