Onze [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) heeft een krachtige importmechanisme op basis van een UI. Na het installeren van de plugin
wordt u begeleid bij het koppelen van uw WordPress-installatie aan FastComments en het overzetten van uw bestaande commentaargegevens.

**Dit gebeurt zonder dat u iets handmatig hoeft te kopiëren of te downloaden.**

Het migratieproces wordt via de UI aan u getoond tijdens de migratie. De meeste migraties duren slechts een paar minuten.

Het mechanisme is ontworpen om uw WordPress-installatie niet onnodig te belasten tijdens de migratie.

Als u uw site van WordPress verhuist, kunt u in plaats van de plugin een WordPress XML- of CSV-export importeren. Zie
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Firewalls

Om de geautomatiseerde WordPress-installatie te laten werken, moeten we aanroepen doen naar uw WordPress-installatie.
Firewalls zoals Cloudflare kunnen ons blokkeren en ervoor zorgen dat de integratie mislukt. In dat geval [kunnen wij u](https://fastcomments.com/auth/my-account/help) een set IP-adressen geven die u moet whitelisten voor de integratie.

### Datagebruik

In het geval van onze WordPress-migratie worden nieuwe of bijgewerkte commentaargegevens automatisch op de achtergrond teruggesynchroniseerd naar uw WordPress-installatie.
Dit betekent dat, terwijl de reacties door FastComments zelf worden aangeboden om de belasting van uw WordPress-implementatie te verminderen,
wij ze **ook** opslaan in uw database als back-up. Dit betekent ook dat, als u besluit weg te gaan bij FastComments, uw gegevens
al zijn gemigreerd en up-to-date zijn.