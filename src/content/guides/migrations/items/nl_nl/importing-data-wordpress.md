Onze [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) heeft een krachtig UI-gebaseerd importmechanisme. Na het installeren van de plugin,
zal het je begeleiden bij het koppelen van je WordPress‑installatie met FastComments en het kopiëren van je bestaande commentaargegevens.

**Dit gebeurt zonder handmatig iets te kopiëren of te downloaden.**

Het migratieproces wordt je via de UI aangegeven tijdens de migratie. De meeste migraties duren slechts een paar minuten.

Het mechanisme is ontworpen om geen overmatige belasting op je WordPress‑installatie te veroorzaken tijdens de migratie.

Als je je site van WordPress verhuist, kun je een WordPress XML‑ of CSV‑export importeren in plaats van de plugin te gebruiken. Zie
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Om de geautomatiseerde WordPress‑configuratie te laten werken, moeten we oproepen doen naar je WordPress‑installatie.
Firewalls zoals Cloudflare kunnen ons blokkeren en de integratie laten mislukken. In dergelijke gevallen kunnen we je [we can provide you](https://fastcomments.com/auth/my-account/help) voorzien van een reeks IP’s om te whitelisten voor de integratie.

### Data Ownership

In het geval van onze WordPress‑migratie wordt alle nieuwe of bijgewerkte commentaardata automatisch op de achtergrond gesynchroniseerd met je WordPress‑installatie. Dit betekent dat, terwijl de opmerkingen door FastComments zelf worden geleverd om de belasting van je WordPress‑implementatie te verlagen,
we **also** ook opslaan in je database als back‑up. Dit betekent ook dat als je wilt overstappen van FastComments, je gegevens al gemigreerd en up‑to‑date zijn.