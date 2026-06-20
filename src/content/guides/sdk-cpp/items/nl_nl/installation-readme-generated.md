### Afhankelijkheden installeren

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Bouwen vanaf broncode

```bash
mkdir build
cd build
cmake ..
make
```

### Installeren

```bash
sudo make install
```

### Inhoud van de bibliotheek

Deze bibliotheek bevat de gegenereerde API-client en de SSO-hulpmiddelen om het werken met de API eenvoudiger te maken.

- [Documentatie van de API-clientbibliotheek](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Publieke vs Beveiligde API's

Voor de API-client zijn er drie classes, `DefaultApi`, `PublicApi`, en `ModerationApi`. De `DefaultApi` bevat methoden die uw API-sleutel vereisen, en `PublicApi` bevat
methoden die rechtstreeks vanuit een browser/mobiel apparaat/etc. kunnen worden aangeroepen zonder authenticatie. De `ModerationApi` bevat methoden die het moderatiedashboard aansturen - lijstweergave,
tellen, zoeken, exporteren en het ophalen van logs voor reacties, moderatieacties (verwijderen/terugzetten, markeren, review/spam/goedkeuringsstatus instellen, stemmen aanpassen, draadjes heropenen/sluiten),
verbanningen (verbanning op basis van een reactie, bans ongedaan maken, pre-ban-samenvattingen, banstatus en voorkeuren, aantallen verbannen gebruikers), en badges & vertrouwen (badges toekennen/verwijderen, handmatige badges, get/set trust
factor, intern gebruikersprofiel). Elke `ModerationApi`-methode accepteert een `sso`-parameter zodat de aanroep wordt uitgevoerd namens een via SSO geauthenticeerde moderator.