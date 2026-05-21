### Installeren vanuit de Shopify App Store

1. Open de [FastComments-pagina in de Shopify App Store](https://apps.shopify.com/fastcomments).
2. Klik op **Add app** en kies tijdens de installatieflow het gewenste abonnement.
3. Shopify verwijst je terug naar de FastComments-beheeromgeving binnen Shopify wanneer de installatie is voltooid.

Dat is de volledige installatie. Er hoeft niets in je thema-bestanden geplakt te worden.

### Wat er voor je wordt ingesteld

De installatie voert alles uit wat je anders handmatig zou moeten doen:

- Er wordt een FastComments-tenant aangemaakt voor je winkel en gekoppeld aan je shopdomein.
- De store-URL van je shop wordt toegevoegd aan de geautoriseerde domeinen van de tenant, zodat reacties laden zonder domeinfout.
- Er wordt een `fastcomments.tenant_id` shop-metavelden geschreven zodat elk blok weet tegen welke tenant het moet renderen.
- Single sign-on voor je Shopify-klanten is standaard ingeschakeld.
- De facturering loopt via Shopify Managed Pricing. Kosten verschijnen op je reguliere Shopify-factuur. Upgrade, downgrade of annuleer via **Instellingen > Apps en verkoopkanalen > FastComments** in je Shopify-beheer.

Als je shop al klant was bij FastComments voordat je de app installeerde, hergebruikt de installatie de bestaande tenant in plaats van een nieuwe aan te maken.

### De ingebedde beheeromgeving

Wanneer je de FastComments-app vanuit je Shopify-beheer opent, kom je op een dashboard met één-klik-tegels naar de volledige FastComments-backend:

- **Dashboard**: accountinstellingen, gebruik en abonnementsgegevens.
- **Moderation Queue**: reacties door je hele winkel goedkeuren, afwijzen en beantwoorden.
- **Customize**: widgetkleuren, lettertypen, moderatieregels en configuratie aanpassen.
- **Ratings & Reviews Helper**: sterbeoordelingen en reviewvragen instellen als je de Reviews Summary-blok wilt gebruiken.

Elke tegel opent FastComments met een éénmalige loginlink, zodat je geen apart aanmeldaccount nodig hebt.

### Volgende stap: blokken toevoegen aan je winkel

Open je Shopify-thema-editor (**Online Store > Themes > Customize**), open de template waarin je reacties of beoordelingen wilt toevoegen en klik op **Add block**. De FastComments-blokken verschijnen onder **Apps**. De rest van deze gids behandelt elk blok afzonderlijk.