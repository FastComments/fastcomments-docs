### PyPI

```bash
pip install fastcomments
```

### Biblioteksindhold

Dette bibliotek indeholder to moduler: den genererede API-klient og kernebiblioteket til Python, som indeholder håndskrevne hjælpefunktioner for at gøre arbejdet med API'et lettere, inklusive SSO-understøttelse.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Kernebibliotekets dokumentation, inklusive SSO-eksempler](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Offentlige vs. sikrede API'er

For API-klienten er der tre klasser, `DefaultApi`, `PublicApi`, og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder metoder, som kan bruges direkte fra en browser/mobilenhed/etc. uden autentificering. `ModerationApi` driver moderatorpanelet og indeholder metoder til moderering af kommentarer (liste, optælling, søgning, logfiler, eksport), moderationhandlinger (fjern/gendan, marker, indstil gennemgang/spam/godkendelsesstatus, stemmer, genåbn/luk tråd), udelukkelser (udeluk fra at kommentere, fortryd, forudgående udelukkelsesoversigter, udelukkelsesstatus/indstillinger, antal udelukkede brugere) og badges & tillid (tildel/fjern badge, manuelle badges, hent/indstil tillidsfaktor, brugerens interne profil). Hver `ModerationApi`-metode accepterer en `sso`-parameter, så den kan kaldes på vegne af en SSO-autentificeret moderator.