### Korištenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvornog koda

```bash
nimble build
```

### Sadržaj knjižnice

Ova knjižnica sadrži generiranog API klijenta i SSO alate koji olakšavaju rad s API‑jem.

- [Dokumentacija API klijentske knjižnice](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javne vs. zaštićene API‑je

Za API klijenta postoje tri API modula, `api_default`, `api_public` i `api_moderation`. `api_default` sadrži metode koje zahtijevaju vaš API ključ, a `api_public` sadrži API pozive koji se mogu izravno izvršiti iz preglednika/mobilnog uređaja itd. bez autentifikacije. Modul `api_moderation` sadrži metode za nadzornu nadzornu ploču.

Modul `api_moderation` pruža opsežan skup API‑ja za brzu i neposrednu moderaciju. Svaka metoda `api_moderation` prihvaća parametar `sso` i može se autentificirati putem SSO‑a ili kolačića sesije FastComments.com.