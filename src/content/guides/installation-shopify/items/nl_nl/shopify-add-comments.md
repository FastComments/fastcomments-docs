Het **FastComments**-blok is de hoofdwidget voor reacties. Voeg het toe aan blogpost-templates, product-templates, of elke andere pagina waar je een discussiethread of een live chat wilt.

### Het blok toevoegen

1. Open de Shopify theme editor (**Online Store > Themes > Customize**).
2. Kies de template waar je reacties op wilt: **Blog post**, **Product**, of een andere pagina- of sectie-template.
3. Klik in de sectie waar je reacties wilt laten verschijnen op **Add block**.
4. Onder **Apps**, selecteer **FastComments**.
5. Klik op **Save**.

Het blok verschijnt direct. Er hoeft geen Tenant ID te worden ingevuld; de tenant van je winkel wordt automatisch gekoppeld wanneer je de app installeert.

### Instellingen

| Instelling | Wat het doet | Standaard |
|---|---|---|
| Tenant ID (optioneel) | Overschrijf tegen welke FastComments-tenant het blok rendert. Laat leeg om de automatisch geconfigureerde tenant van de winkel te gebruiken. Een handmatige tenant ID vind je op fastcomments.com/auth/my-account/api-secret. | (leeg) |
| SSO | Logt de bezoeker automatisch in met zijn Shopify-klantaccount voordat er gereageerd wordt. Zie [Auto-Login Shopify Customers](/guide-installation-shopify.html#shopify-sso). | Aan |
| Commenting Style | **Threaded** voor geneste reacties en stemmen, of **Streaming** voor een realtime chatfeed. | Threaded |
| Custom URL ID | Overschrijf de automatisch gedetecteerde pagina-identificatie. Gebruik dit wanneer je wilt dat twee URL's één commentaardelen. | (automatisch gedetecteerd) |

### Hoe de pagina-identificatie wordt gekozen

Elke commentaarthread is gekoppeld aan een URL ID. Het blok kiest er automatisch één:

- **Blogpost-template:** `shopify-article-{article.id}`, wat stabiel blijft bij wijzigingen in slug en titel.
- **Product-template:** `shopify-product-{product.id}`, wat stabiel blijft bij wijzigingen in slug en titel.
- **Andere templates:** het request-pad.

Als je **Custom URL ID** instelt, wordt die waarde in plaats daarvan gebruikt. Gebruik dezelfde Custom URL ID over meerdere blokken (bijvoorbeeld op een gelokaliseerde variant van een productpagina) om één commentaarthread te delen.

### Threaded vs Streaming

**Threaded** is de standaard. Bezoekers reageren op elkaar, kunnen stemmen en de moderatietools werken zoals verwacht. Ideaal voor blogposts en productreviews.

**Streaming** laat het threaden vallen en toont nieuwe reacties in realtime zodra ze geplaatst worden, zoals een chatfeed. Ideaal voor productlanceringen, live events en communitypagina's.

### Meerdere blokken op dezelfde pagina

Het blok kan meer dan eens aan dezelfde template worden toegevoegd. Bijvoorbeeld een Reviews Summary bovenaan een productpagina en een FastComments-blok onderaan. De blokken delen een URL ID, dus de samenvatting weerspiegelt de reacties eronder.

### Tips

- Het blok verbergt zichzelf in de theme editor preview met een gele melding als het geen tenant kan vinden. Als dat in je live winkel verschijnt, installeer de FastComments-app opnieuw.
- Voor een productpagina dient het FastComments-blok tevens als je productreviews-widget. Combineer het met **FastComments - Reviews Summary** voor een sterrenbeoordelingssamenvatting bovenaan de pagina.