FastComments Rust SDK består af flere moduler:

- **Client-modul** - API-klient for FastComments REST API'er
  - Fuldstændige typedefinitioner for alle API-modeller
  - Tre API-klienter, der dækker alle FastComments-metoder:
    - `default_api` (**DefaultApi**) - Metoder autentificeret med API-nøgle til brug på serversiden
    - `public_api` (**PublicApi**) - Offentlige metoder uden API-nøgle, som er sikre at kalde fra browsere og mobilapps
    - `moderation_api` (**ModerationApi**) - Metoder bag moderator-dashboardet, inklusive kommentarmoderation (liste, optælling, søgning, logfiler, eksport), moderationhandlinger (fjern/gendan, flag, angiv gennemgang-/spam-/godkendelsesstatus, stemmer, genåbn/luk tråd), udvisninger (udvis fra en kommentar, fortryd, forudgående udvisningsoversigter, udvisningsstatus/præferencer, antal udviste brugere) og badges & tillid (tildel/fjern badges, manuelle badges, hent/angiv tillidsfaktor, brugerens interne profil). Hver Moderation-metode accepterer en `sso`-parameter, så kaldet kan udføres på vegne af en SSO-godkendt moderator.
  - Fuld async/await-understøttelse med tokio
  - Se [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) for detaljeret API-dokumentation

- **SSO-modul** - Serverside Single Sign-On-værktøjer
  - Sikker token-generering til brugergodkendelse
  - Understøttelse af både simpel og sikker SSO-tilstand
  - Token-signering baseret på HMAC-SHA256

- **Core-typer** - Delte typedefinitioner og værktøjer
  - Kommentar-modeller og metadata-strukturer
  - Bruger- og lejerkonfigurationer
  - Hjælpefunktioner til almindelige operationer