Het **FastComments - Comment Count**-blok toont een klein aantal reacties voor één pagina. Gebruik het in blogpostlijsten, productkaarten of elke template die linkt naar een pagina met reacties, zodat bezoekers kunnen zien hoe actief elke discussie is voordat ze doorklikken.

### Het blok toevoegen

1. Open de Shopify-thema-editor.
2. Open de template waar je het aantal wilt laten verschijnen. Bijvoorbeeld de **Blog**-template (de postlijst) of een productoverzichtsectie.
3. Klik op **Add block** in de sectie die elk item weergeeft.
4. Onder **Apps**, selecteer **FastComments - Comment Count**.
5. Klik op **Save**.

### Instellingen

| Instelling | Wat het doet | Standaard |
|---|---|---|
| Tenant ID (optioneel) | Overschrijf van welke FastComments-tenant het aantal wordt gelezen. Laat leeg om de automatisch geconfigureerde tenant van de winkel te gebruiken. | (leeg) |
| Custom URL ID | Overschrijf de pagina-identificatie waar het aantal naar zoekt. Gebruik dit wanneer het aantal zich op een andere pagina bevindt dan het FastComments-blok dat het volgt. | (automatisch gedetecteerd) |

### Hoe het aantal overeenkomt met de discussie

Het Comment Count-blok gebruikt dezelfde auto-detectielogica als het **FastComments**-blok:

- Blogpost-template: `shopify-article-{article.id}`
- Product-template: `shopify-product-{product.id}`
- Andere templates: het verzoekpad

Als je een **Custom URL ID** instelt op het **FastComments**-blok op een pagina, stel dan dezelfde Custom URL ID in op het Comment Count-blok zodat ze naar dezelfde discussie verwijzen.

### Tips

- De aantallen voor elk item op de pagina worden in één verzoek opgehaald, dus het toevoegen van het blok aan elk item in een lange lijst heeft geen extra round-trip-kosten.
- Eén Comment Count-blok per artikel of product in een overzicht is het verwachte gebruik; het blok kan zo vaak worden toegevoegd als je nodig hebt.