### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane pomoćne alatke koje olakšavaju rad sa API-jem, uključujući podršku za SSO.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući primere za SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni naspram zaštićenih API-ja

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi`, i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozivati direktno iz pregledača/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` pokreće kontrolnu tablu moderatora i sadrži metode za moderisanje komentara (prikaz, brojanje, pretraga, logovi, izvoz), akcije moderacije (ukloni/vrati, označi, postavi status pregleda/spama/odobrenja, glasovi, ponovo otvori/zatvori nit), zabrane (zabrana komentara, poništi, sažeci pre zabrane, status/preferencije zabrane, brojevi zabranjenih korisnika), i značke & poverenje (dodeli/ukloni značku, ručno dodeljene značke, dohvati/postavi faktor poverenja, interni korisnički profil). Svaka metoda `ModerationApi` prihvata parametar `sso` kako bi se mogla pozvati u ime moderatora autentifikovanog putem SSO.