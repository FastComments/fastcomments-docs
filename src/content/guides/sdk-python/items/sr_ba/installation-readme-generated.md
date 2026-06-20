### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane pomoćne funkcije koje olakšavaju rad sa API-jem, uključujući podršku za SSO.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući primjere SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pokreće moderatorski kontrolni panel i sadrži metode za moderisanje komentara (lista, brojanje, pretraga, logovi, izvoz), akcije moderiranja (uklanjanje/obnavljanje, prijava, postavljanje statusa pregleda/spama/odobravanja, glasovi, ponovno otvaranje/zatvaranje teme), zabrane (zabrana komentarisanja, poništavanje, sažeci prije zabrane, status/preferencije zabrane, broj zabranjenih korisnika) i značke i povjerenje (dodjela/uklanjanje značke, ručne značke, dohvatanje/postavljanje faktora povjerenja, interni korisnički profil). Svaka metoda `ModerationApi` prihvata parametar `sso` tako da se može pozvati u ime moderatora autentifikovanog preko SSO.