---
### Nimble gebruiken

```bash
nimble install fastcomments
```

### Bouwen vanuit bron

```bash
nimble build
```

### Inhoud van de bibliotheek

Deze bibliotheek bevat de tegenereerde API-client en de SSO-hulpmiddelen om het werken met de API gemakkelijker te maken.

- [Documentatie API-clientbibliotheek](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Openbare vs beveiligde API's

Voor de API-client zijn er drie API-modules, `api_default`, `api_public` en `api_moderation`. De `api_default` bevat methoden die uw API-sleutel vereisen, en `api_public` bevat API-aanroepen die rechtstreeks vanuit een browser/mobiel apparaat/etc. kunnen worden gedaan zonder authenticatie. De `api_moderation`-module bevat methoden voor het moderator-dashboard.

De `api_moderation`-module biedt een uitgebreide reeks live en snelle moderatie-API's. Elke `api_moderation`-methode accepteert een `sso`-parameter en kan authenticeren via SSO of een FastComments.com sessiecookie.
---