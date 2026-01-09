### Korištenje Nimble

```bash
nimble install fastcomments
```

### Sastavljanje iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generirani API klijent i SSO alate koji olakšavaju rad s API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje dva API modula, `api_default` i `api_public`. `api_default` sadrži metode koje zahtijevaju vaš API ključ, a `api_public` sadrži pozive API-ja koje je moguće izvršiti izravno iz preglednika/mobilnog uređaja/itd. bez autentikacije.