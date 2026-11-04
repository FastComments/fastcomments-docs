Het **FastComments - Reviews Summary** blok toont een geaggregeerde sterrenbeoordeling en een overzicht van reviews voor een pagina. Combineer het met het **FastComments** blok op producttemplates voor de standaard reviews-layout: samenvatting bovenaan, reviewformulier en reviews eronder.

### Vereiste: stel Ratings & Reviews in

Het Reviews Summary blok toont de beoordelingsvragen die je voor je winkel hebt geconfigureerd. Stel die eerst in:

1. Open de FastComments app in je Shopify-beheerdersomgeving.
2. Klik op de tegel **Ratings & Reviews Helper** (of open [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) rechtstreeks).
3. Voeg de vragen toe die elke reviewer moet beantwoorden (algemene sterrenbeoordeling, "hoe was de pasvorm", enz.).

Zonder geconfigureerde vragen heeft het samenvattingsblok niets om samen te vatten.

### Voeg het blok toe

1. Open de Shopify theme editor.
2. Open de **Product** template (of de paginatemplate waar je de samenvatting wilt).
3. Klik op **Add block** bovenaan de paginasectie, boven de plek waar het **FastComments** blok komt.
4. Kies onder **Apps** voor **FastComments - Reviews Summary**.
5. Voeg lager op dezelfde pagina een **FastComments** blok toe als je die nog niet hebt, zodat bezoekers reviews kunnen achterlaten.
6. Klik op **Save**.

### Instellingen

| Instelling | Wat het doet | Standaard |
|---|---|---|
| Tenant ID (optioneel) | Overschrijf van welke FastComments tenant de samenvatting leest. Laat leeg om de automatisch geconfigureerde tenant van de winkel te gebruiken. | (leeg) |
| Custom URL ID | Overschrijf de pagina-identificatie waarop de samenvatting aggregeert. Gebruik dit wanneer de samenvatting op een andere pagina staat dan het FastComments blok waarop deze betrekking heeft. | (automatisch gedetecteerd) |

### Hoe de samenvatting de reviews matcht

Het Reviews Summary blok gebruikt dezelfde auto-detectielogica als het **FastComments** blok:

- Producttemplate: `shopify-product-{product.id}`
- Blogposttemplate: `shopify-article-{article.id}`
- Andere templates: het request-pad

Op een normale productpagina delen de samenvatting en de commentthread automatisch een URL ID, zonder configuratie.

### Tips

- De samenvatting is read-only. Om reviews te verzamelen, heb je een **FastComments** blok op dezelfde pagina nodig.
- Als je beoordelingsvragen in Ratings & Reviews Helper wijzigt nadat er reviews zijn verzameld, berekent de samenvatting opnieuw op basis van de nieuwe set vragen.