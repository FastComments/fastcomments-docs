### Korišćenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvora

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisani API klijent i SSO alate koji olakšavaju rad sa API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni vs Zaštićeni API-ji

Za API klijent postoje tri API modula, `api_default`, `api_public` i `api_moderation`. Modul `api_default` sadrži metode koje zahtevaju vaš API ključ, a `api_public` sadrži API pozive koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. Modul `api_moderation` sadrži metode za moderatorsku kontrolnu tablu.

Modul `api_moderation` pruža opsežan skup API-ja za live i brzu moderaciju. Svaka metoda `api_moderation` prihvata `sso` parametar i može se autentifikovati putem SSO ili FastComments.com sesijskog kolačića.