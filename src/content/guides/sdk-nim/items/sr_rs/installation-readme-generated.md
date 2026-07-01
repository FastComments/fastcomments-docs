### Korišćenje Nimble

```bash
nimble install fastcomments
```

### Građenje iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisanog API klijenta i SSO alate kako bi rad sa API‑jem bio jednostavniji.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni vs zaštićeni API‑ji

Za klijenta API‑ja, postoje tri modula API‑ja, `api_default`, `api_public` i `api_moderation`. `api_default` sadrži metode koje zahtevaju vaš API ključ, a `api_public` sadrži API pozive koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd bez autentifikacije. `api_moderation` modul sadrži metode za moderacijsku kontrolnu tablu.

`api_moderation` modul pruža opsežan skup live i brzih moderacijskih API‑ja. Svaka metoda `api_moderation` prihvata parametar `sso` i može da se autentifikuje putem SSO‑a ili FastComments.com sesijskog kolačića.