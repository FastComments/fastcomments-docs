Næste skal vi rulle ned til linje `100`:

<div class="screenshot white-bg">
    <div class="title">Rul til linje 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Rul til linje 100" />
</div>

Kopier nu følgende kodeudsnit, som er designet **specifikt til Shopify - brug ikke kodeudsnit fra andre vejledninger**:

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

Nu vil vi placere markøren på `line 101` - lige efter `</div>` - og indsætte. Du bør have noget som dette:

<div class="screenshot white-bg">
    <div class="title">Tilføj FastComments-koden</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Tilføj FastComments-koden" />
</div>

Nu kan vi gemme:

<div class="screenshot white-bg">
    <div class="title">Gem</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Gem" />
</div>