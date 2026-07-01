### Korišćenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisani API klijent i SSO alate koji olakšavaju rad s API‑jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni vs Zaštićeni API‑jevi

Za API klijent postoje tri API modula: `api_default`, `api_public` i `api_moderation`. `api_default` sadrži metode koje zahtijevaju vaš API ključ, a `api_public` sadrži API pozive koji se mogu izvesti direktno iz preglednika/mobilnog uređaja itd. bez autentifikacije. Modul `api_moderation` sadrži metode za moderator kontrolnu tablu.

Modul `api_moderation` pruža opsežan skup API‑ja za živu i brzu moderaciju. Svaka metoda `api_moderation` prihvata parametar `sso` i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.