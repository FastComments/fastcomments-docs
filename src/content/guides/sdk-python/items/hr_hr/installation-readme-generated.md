### PyPI

```bash
pip install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generirani API klijent i temeljna Python biblioteka koja sadrži ručno napisane pomoćne funkcije koje olakšavaju rad s API-jem, uključujući podršku za SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni i zaštićeni API-ji

Za API klijent postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API key, a `PublicApi` sadrži API pozive koji se mogu izvršiti izravno iz preglednika/mobilnog uređaja/itd bez autentikacije.