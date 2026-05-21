De **FastComments**-blok is de hoofdreactie-widget. Voeg het toe aan blogpost-sjablonen, productsjablonen of elke andere pagina waar je een discussiedraad of een livechat wilt.

### Blok toevoegen

1. Open de Shopify-thema-editor (**Online Store > Themes > Customize**).
2. Kies het sjabloon waar je reacties op wilt: **Blog post**, **Product**, of een ander pagina- of sectiesjabloon.
3. Klik in de sectie waar je reacties wilt laten verschijnen op **Add block**.
4. Kies onder **Apps** voor **FastComments**.
5. Klik op **Save**.

Het blok verschijnt onmiddellijk. Er is geen Tenant ID om in te voeren; de tenant van je winkel wordt automatisch gekoppeld wanneer je de app installeert.

### Instellingen

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Overschrijf welke FastComments-tenant het blok weergeeft. Laat leeg om de automatisch geconfigureerde tenant van de winkel te gebruiken. Vind een handmatig tenant ID op fastcomments.com/auth/my-account/api-secret. | (leeg) |
| SSO | Logt de bezoeker automatisch in als hun Shopify-klantaccount voordat ze reageren. Zie [Automatisch inloggen van Shopify-klanten](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** voor geneste antwoorden en stemmen, of **Streaming** voor een realtime chatfeed. | Threaded |
| Custom URL ID | Overschrijf de automatisch gedetecteerde pagina-identificatie. Gebruik dit wanneer je wilt dat twee URL's één reactiedraad delen. | (automatisch gedetecteerd) |

### Hoe de pagina-identificatie wordt gekozen

Elke reactiedraad heeft een sleutel op basis van een URL-ID. Het blok kiest er automatisch één:

- **Blog post template:** `shopify-article-{article.id}`, which is stable across slug and title changes.
- **Product template:** `shopify-product-{product.id}`, which is stable across slug and title changes.
- **Other templates:** the request path.

Als je **Custom URL ID** instelt, wordt die waarde in plaats daarvan gebruikt. Gebruik dezelfde Custom URL ID in meerdere blokken (bijvoorbeeld op een gelokaliseerde variant van een productpagina) om één reactiedraad te delen.

### Threaded vs Streaming

**Threaded** is de standaard. Bezoekers reageren op elkaar, stemmen en moderatietools werken zoals verwacht. Het beste voor blogposts en productrecensies.

**Streaming** laat threading weg en toont nieuwe opmerkingen realtime zodra ze geplaatst worden, zoals een chatfeed. Het beste voor productlanceringen, live-evenementen en communitypagina's.

### Meerdere blokken op dezelfde pagina

Het blok kan meer dan eens worden toegevoegd aan hetzelfde sjabloon. Bijvoorbeeld een Reviews Summary bovenaan een productpagina en een FastComments-blok onderaan. De blokken delen een URL-ID, dus de samenvatting weerspiegelt de opmerkingen hieronder.

### Tips

- Het blok verbergt zichzelf in de thema-editor-voorbeeldweergave met een gele melding als het geen tenant kan vinden. Als dat in je live-winkel verschijnt, installeer de FastComments-app opnieuw.
- Voor een productpagina fungeert het FastComments-blok tevens als je widget voor productrecensies. Combineer het met **FastComments - Reviews Summary** voor een sterrenbeoordelingssamenvatting bovenaan de pagina.