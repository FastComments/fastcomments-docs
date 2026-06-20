### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane pomoćne alate koji olakšavaju rad sa API-jem, uključujući podršku za SSO.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući SSO primjere](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni naspram zaštićenih API-ja

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi`, i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozivati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pokreće kontrolnu tablu moderatora i sadrži metode za moderaciju komentara (lista, brojanje, pretraga, zapisi, izvoz), akcije moderacije (ukloni/vrati, prijavi, postavi status za pregled/spam/odobrenje, glasovi, ponovo otvori/zatvori nit), zabrane (zabrana komentarisanja, poništi, rezimei pre-banovanja, status/preferencije zabrane, brojevi zabranjenih korisnika), i značke & poverenje (dodeli/ukloni značku, ručne značke, preuzmi/postavi faktor poverenja, interna korisnička profil). Svaka metoda `ModerationApi` prihvata parametar `sso` kako bi mogla biti pozvana u ime moderatora autentifikovanog putem SSO.