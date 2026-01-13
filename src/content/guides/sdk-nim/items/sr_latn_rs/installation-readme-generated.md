---
### Korišćenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO alatke koje olakšavaju rad sa API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni naspram zaštićenih API-ja

Za API klijenta postoje dva API modula, `api_default` i `api_public`. `api_default` sadrži metode koje zahtevaju vaš API ključ, a `api_public` sadrži API pozive
koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.
---