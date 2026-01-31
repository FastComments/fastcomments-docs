Vervolgens scrollen we naar regel `100`:

<div class="screenshot white-bg">
    <div class="title">Scroll naar regel 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Scroll naar regel 100" />
</div>

Kopieer nu het volgende codefragment, dat **speciaal voor Shopify is ontworpen - gebruik geen codefragmenten uit andere handleidingen**:

[inline-code-attrs-start title = 'Shopify FastComments-fragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zet nu de cursor op `line 101` - direct na de `</div>` - en plak. Je zou zoiets moeten hebben:

<div class="screenshot white-bg">
    <div class="title">Voeg de FastComments-code toe</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Voeg de FastComments-code toe" />
</div>

Nu kunnen we opslaan:

<div class="screenshot white-bg">
    <div class="title">Opslaan</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Opslaan" />
</div>

---