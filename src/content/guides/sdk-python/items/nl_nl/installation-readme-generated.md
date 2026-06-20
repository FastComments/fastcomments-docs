### PyPI

```bash
pip install fastcomments
```

### Inhoud van de bibliotheek

Deze bibliotheek bevat twee modules: de gegenereerde API-client en de core Python-bibliotheek die handgeschreven hulpprogramma's bevat om het werken met de API eenvoudiger te maken, inclusief SSO-ondersteuning.

- [Documentatie API-clientbibliotheek](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentatie core-bibliotheek, inclusief SSO-voorbeelden](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Openbare vs beveiligde API's

Voor de API-client zijn er drie klassen, `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API-sleutel vereisen, en `PublicApi` bevat methoden die direct vanuit een browser/mobiel apparaat/etc. zonder authenticatie kunnen worden aangeroepen. De `ModerationApi` voedt het moderatordashboard en bevat methoden voor het modereren van opmerkingen (lijst, aantal, zoeken, logs, export), moderatieacties (verwijderen/herstellen, markeren, instellen van review/spam/goedkeuringsstatus, stemmen, draad heropenen/sluiten), verbanningen (verbannen om te reageren, ongedaan maken, pre-ban-samenvattingen, ban-status/voorkeuren, aantallen verbannen gebruikers), en badges & vertrouwen (badge toekennen/verwijderen, handmatige badges, ophalen/instellen van vertrouwensfactor, intern gebruikersprofiel). Elke `ModerationApi`-methode accepteert een `sso`-parameter zodat deze kan worden aangeroepen namens een via SSO geauthenticeerde moderator.