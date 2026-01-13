Vervolgens gaan we naar regel `100` scrollen:

<div class="screenshot white-bg">
    <div class="title">Scroll naar regel 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Scroll naar regel 100" />
</div>

Kopieer nu het volgende codefragment, dat **specifiek voor Shopify is ontworpen - gebruik geen codefragmenten uit andere tutorials**:

[inline-code-attrs-start title = 'Shopify FastComments-fragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

Nu willen we onze cursor op `regel 101` plaatsen - direct na de `</div>` - en plakken. Je zou iets zoals dit moeten hebben:

<div class="screenshot white-bg">
    <div class="title">Voeg de FastComments-code toe</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Voeg de FastComments-code toe" />
</div>

Nu kunnen we opslaan:

<div class="screenshot white-bg">
    <div class="title">Opslaan</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Opslaan" />
</div>