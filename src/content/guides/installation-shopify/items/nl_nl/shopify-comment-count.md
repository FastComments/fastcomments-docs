De **FastComments - Comment Count** block toont een klein aantal reacties voor een enkele pagina. Gebruik het in blogpostlijsten, productkaarten of elk template dat naar een pagina met reacties linkt, zodat bezoekers kunnen zien hoe actief elk onderwerp is voordat ze doorklikken.

### Add the block

1. Open de Shopify thema-editor.
2. Open het template waar je het aantal wilt laten verschijnen. Bijvoorbeeld de **Blog** template (de berichtlijst) of een productlijstsectie.
3. Klik op **Add block** in de sectie die elk item weergeeft.
4. Onder **Apps**, selecteer **FastComments - Comment Count**.
5. Klik op **Save**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Overschrijf van welke FastComments-tenant het aantal wordt gelezen. Laat leeg om de automatisch geconfigureerde tenant van de winkel te gebruiken. | (blank) |
| Custom URL ID | Overschrijf de pagina-identifier waarnaar het aantal zoekt. Gebruik dit wanneer het aantal zich op een andere pagina bevindt dan het FastComments-blok dat het volgt. | (auto-detected) |

### How the count matches the comment thread

De Comment Count block gebruikt dezelfde autodetectielogica als het **FastComments**-blok:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

Als je een **Custom URL ID** instelt op het **FastComments**-blok op een pagina, stel dan dezelfde Custom URL ID in op het Comment Count-blok zodat ze naar hetzelfde onderwerp verwijzen.

### Tips

- Aantallen voor elk item op de pagina worden in één verzoek opgehaald, dus het toevoegen van het blok aan elk item in een lange lijst veroorzaakt geen extra netwerkverkeer.
- Eén Comment Count-blok per artikel of product in een lijst is het verwachte gebruik; het blok kan zo vaak worden toegevoegd als je nodig hebt.