### Using Nimble

```bash
nimble install fastcomments
```

### Building from Source

```bash
nimble build
```

### Library Contents

Ova biblioteka sadrži generisani API klijent i SSO alate koji olakšavaju rad s API‑jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Public vs Secured APIs

Za API klijent postoje tri API modula: `api_default`, `api_public` i `api_moderation`. `api_default` sadrži metode koje zahtijevaju vaš API ključ, a `api_public` sadrži API pozive koji se mogu izvršiti direktno iz preglednika/mobild uređaja itd. bez autentifikacije. Modul `api_moderation` sadrži metode za moderacijski dashboard.

Modul `api_moderation` pruža opsežan skup live i brzih moderacijskih API‑ja. Svaka metoda `api_moderation` prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com kolačića sesije.