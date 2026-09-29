Onze [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) heeft een krachtig UI‑gebaseerd importmechanisme. Na het installeren van de plugin,
wordt je begeleid bij het koppelen van je WordPress‑installatie met FastComments en het overzetten van je bestaande commentaargegevens.

**Dit gebeurt zonder handmatig iets te kopiëren of te downloaden.**

Het migratieproces wordt via de UI aangegeven tijdens de migratie. De meeste migraties duren slechts een paar minuten.

Het mechanisme is ontworpen om geen overmatige belasting op je WordPress‑installatie te veroorzaken tijdens de migratie.

Als je je site van WordPress wilt verplaatsen, kun je een WordPress XML‑ of CSV‑export importeren in plaats van de plugin te gebruiken. Zie
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Om de geautomatiseerde WordPress‑setup te laten werken, moeten we oproepen doen naar je WordPress‑installatie.
Firewalls zoals Cloudflare kunnen ons blokkeren en de integratie laten mislukken. In dergelijke gevallen [kunnen we je
voorzien](https://fastcomments.com/auth/my-account/help) van een reeks IP‑adressen om op de whitelist te zetten voor de integratie.

### Data Ownership

In het geval van onze WordPress‑migratie wordt elke nieuwe of bijgewerkte commentaardata automatisch gesynchroniseerd terug naar je WordPress‑installatie
achter de schermen. Dit betekent dat, terwijl de reacties door FastComments zelf worden geleverd om de belasting van je WordPress‑implementatie te verlagen,
we **ook** een backup in je database opslaan. Dit betekent ook dat als je wilt overstappen van FastComments, je gegevens al gemigreerd en up‑to‑date zijn.