### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovna Python biblioteka koja sadrži ručno napisane pomoćne funkcije koje olakšavaju rad s API-jem, uključujući podršku za SSO.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući primjere SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni naspram zaštićenih API-ja

Za API klijenta postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži API pozive koji se mogu izvršiti direktno iz preglednika/mobilnog uređaja/itd. bez autentifikacije.