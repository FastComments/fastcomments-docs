### Nimble gebruiken

```bash
nimble install fastcomments
```

### Bouwen vanaf de bron

```bash
nimble build
```

### Inhoud van de bibliotheek

Deze bibliotheek bevat de gegenereerde API-client en de SSO-hulpprogramma's om het werken met de API gemakkelijker te maken.

- [Documentatie van de API-clientbibliotheek](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Openbare versus beveiligde API's

Voor de API-client zijn er drie API-modules, `api_default`, `api_public`, en `api_moderation`. De `api_default` bevat methoden die uw API-sleutel vereisen, en `api_public` bevat API-aanroepen
die rechtstreeks vanuit een browser/mobiel apparaat/etc. zonder authenticatie kunnen worden gedaan. De `api_moderation`-module bevat methoden voor het moderatoren-dashboard.

De `api_moderation`-methoden omvatten het opsommen, tellen, doorzoeken en exporteren van opmerkingen en hun logboeken; moderatie-acties zoals het verwijderen/herstellen van opmerkingen, markeren, het instellen van review-/spam-/goedkeuringsstatus, het aanpassen van stemmen, en het heropenen/sluiten van threads; bans (het verbannen van een gebruiker van een opmerking, het ongedaan maken van een ban, pre-ban-samenvattingen, ban-status en voorkeuren, en aantallen gebande gebruikers); en badges & vertrouwen (toekennen/verwijderen van een badge, het opsommen van handmatige badges, het verkrijgen/instellen van iemands trust factor, en het ophalen van het interne profiel van een gebruiker). Elke `api_moderation`-methode accepteert een `sso`-parameter zodat de aanroep geauthenticeerd is als een SSO-moderator.