### Korištenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO utilitete koji olakšavaju rad sa API-jem.

- [Dokumentacija biblioteke API klijenta](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje dva API modula, `api_default` i `api_public`. `api_default` sadrži metode koje zahtijevaju vaš API ključ, a `api_public` sadrži API pozive
koji se mogu izvršavati direktno iz preglednika/mobilnog uređaja/itd. bez autentifikacije.