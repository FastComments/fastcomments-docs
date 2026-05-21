Hvis du ikke kan installere [Shopify App Store appen](https://apps.shopify.com/fastcomments), kan du stadig tilføje FastComments ved at redigere dit tema. Denne fremgangsmåde er nyttig, når du vil tilknytte en FastComments-tenant, du allerede ejer, eller når du indlejrer på en Shopify-front, hvor appen ikke er en mulighed.

Installationen via appen er den understøttede vej for de fleste butikker. Brug kun denne metode, hvis appen ikke passer.

### Trin 1: Deaktiver Shopifys indbyggede kommentarer

I din Shopify-administrator skal du gå til **Blog posts > Manage blogs**, åbne hver blog og sætte **Comments are disabled** i højre panel. Gem.

Dette stopper Shopifys indbyggede kommentarer fra at blive vist sammen med FastComments.

### Trin 2: Åbn blog-temaets skabelon

I din Shopify-administrator:

1. Gå til **Online Store > Themes**.
2. Under dit aktuelle tema, klik **Actions > Edit code**.
3. I filbrowseren til venstre, åbn **Sections** og klik `main-article.liquid`.

Dette er den skabelon, Shopify renderer for et enkelt blogindlæg.

### Trin 3: Indsæt FastComments-snippet

Scroll til omtrent linje 100 i `main-article.liquid`, lige efter den afsluttende `</div>` af artiklens brødtekst. Indsæt følgende snippet:

[inline-code-attrs-start title = 'Shopify FastComments-udsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Erstat `"demo"` med dit eget Tenant ID fra [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Klik **Save**.

### Trin 4: Godkend dit butikdomæne

Åbn et blogindlæg på din live-butik. Hvis du ser en autorisationsfejl i stedet for kommentars-widget'en, skal FastComments vide, at din butik må bruge denne tenant. Se [Domænefejl](/guide-installation-shopify.html#shopify-domain-errors).

### Tilføjelse af FastComments til andre sider

Det samme snippet virker på enhver Liquid-skabelon, inklusive produktsider, tilpassede sider og startsiden. Indsæt det, hvor du vil have kommentarer vist, og juster `urlId`, hvis du vil have en stabil identifikator per side (for eksempel `urlId: "{{ product.id }}"` i en produktskabelon).