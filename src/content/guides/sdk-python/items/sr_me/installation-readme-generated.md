### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane pomoćne funkcije koje olakšavaju rad sa API-jem, uključujući podršku za SSO.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući primjere SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni naspram zaštićenih API-ja

Za API klijent postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži API pozive koje je moguće izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.