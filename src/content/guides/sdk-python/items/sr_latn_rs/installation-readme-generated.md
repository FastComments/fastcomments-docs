### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane pomoćne funkcije koje olakšavaju rad sa API-jem, uključujući podršku za SSO.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući primere SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje dve klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, dok `PublicApi` sadrži pozive API-ja koji se mogu pozivati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.