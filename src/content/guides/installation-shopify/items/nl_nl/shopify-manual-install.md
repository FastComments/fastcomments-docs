---
Als je de [Shopify App Store-app](https://apps.shopify.com/fastcomments) niet kunt installeren, kun je FastComments nog steeds toevoegen door je thema te bewerken. Dit pad is nuttig wanneer je een FastComments-tenant die je al bezit wilt aansluiten, of wanneer je embedt op een Shopify-winkel waar de app geen optie is.

De installatie via de app is de ondersteunde methode voor de meeste winkels. Gebruik deze methode alleen als de app niet geschikt is.

### Stap 1: Schakel de native reacties van Shopify uit

In je Shopify-beheer, ga naar **Blogberichten > Blogs beheren**, open elke blog en stel **Comments are disabled** in het rechterpaneel. Sla op.

Dit voorkomt dat de ingebouwde reacties van Shopify naast FastComments worden weergegeven.

### Stap 2: Open het themasjabloon voor blogs

In je Shopify-beheer:

1. Ga naar **Online winkel > Thema's**.
2. Onder je huidige thema klik je op **Acties > Code bewerken**.
3. In de bestandsbrowser aan de linkerkant, open **Sections** en klik op `main-article.liquid`.

Dit is het sjabloon dat Shopify weergeeft voor een enkel blogartikel.

### Stap 3: Plak de FastComments-snippet

Scroll ongeveer naar regel 100 van `main-article.liquid`, direct na de afsluitende `</div>` van de artikeltekst. Plak de volgende snippet:

[inline-code-attrs-start title = 'Shopify FastComments-snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Vervang `"demo"` door je eigen Tenant ID van [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Klik op **Opslaan**.

### Stap 4: Autoriseer je winkeldomein

Open een blogbericht op je live winkel. Als je in plaats van de commentaarwidget een autorisatiefout ziet, moet FastComments weten dat je winkel is toegestaan om deze tenant te gebruiken. Zie [Domeinfouten](/guide-installation-shopify.html#shopify-domain-errors).

### FastComments toevoegen aan andere pagina's

Dezelfde snippet werkt op elk Liquid-sjabloon, inclusief productpagina's, aangepaste pagina's en de startpagina. Plak het waar je wilt dat reacties verschijnen en pas `urlId` aan als je een stabiele identifier per pagina wilt (bijvoorbeeld, `urlId: "{{ product.id }}"` in een producttemplate).

---