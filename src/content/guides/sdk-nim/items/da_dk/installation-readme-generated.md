---
### Brug af Nimble

```bash
nimble install fastcomments
```

### Byg fra kildekode

```bash
nimble build
```

### Bibliotekets indhold

Dette bibliotek indeholder den genererede API-klient og SSO-værktøjerne, som gør det nemmere at arbejde med API'et.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Offentlige vs sikrede API'er

For API-klienten er der tre API-moduler, `api_default`, `api_public` og `api_moderation`. `api_default` indeholder metoder, der kræver din API-nøgle, og `api_public` indeholder api-kald
som kan foretages direkte fra en browser/mobile enhed/andet uden autentificering. `api_moderation`-modulet indeholder metoder til moderator-dashboardet.

`api_moderation`-metoderne dækker oplistning, optælling, søgning og eksport af kommentarer og deres logfiler; moderationshandlinger som fjernelse/gendannelse af kommentarer, markering, indstilling af review/spam/godkendelsesstatus, justering af stemmer og genåbning/lukning af tråde; udelukkelser (udelukke en bruger fra at kommentere, ophæve en udelukkelse, forudgående udelukkelsesoversigter, udelukkelsesstatus og præferencer samt antal udelukkede brugere); og badges & tillid (tildeling/fjernelse af et badge, liste over manuelle badges, hente/indstille en brugers tillidsfaktor og hente en brugers interne profil). Hver `api_moderation`-metode accepterer en `sso`-parameter, så kaldet er autentificeret som en SSO-moderator.
---