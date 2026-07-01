### Brug af Nimble

```bash
nimble install fastcomments
```

### Byg fra kilde

```bash
nimble build
```

### Biblioteksindhold

Dette bibliotek indeholder den genererede API-klient og SSO‑værktøjerne, så det bliver nemmere at arbejde med API'en.

- [API‑klientbiblioteksdokumentation](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Offentlige vs sikrede API'er

For API-klienten er der tre API-moduler, `api_default`, `api_public` og `api_moderation`. `api_default` indeholder metoder, der kræver din API-nøgle, og `api_public` indeholder API-kald, som kan foretages direkte fra en browser/mobil enhed osv. uden autentificering. `api_moderation`-modulet indeholder metoder til moderator-dashboardet.

`api_moderation`-modulet leverer et omfattende sæt af live- og hurtige moderations‑API'er. Hver `api_moderation`-metode accepterer en `sso`‑parameter og kan autentificere via SSO eller en FastComments.com sessions‑cookie.