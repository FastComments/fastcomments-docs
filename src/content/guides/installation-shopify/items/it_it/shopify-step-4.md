Next stiamo per scorrere fino alla riga `100`:

<div class="screenshot white-bg">
    <div class="title">Scorri alla riga 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Scorri alla riga 100" />
</div>

Ora copia il seguente snippet di codice, progettato **specificamente per Shopify - non usare snippet di codice di altri tutorial**:

[inline-code-attrs-start title = 'Snippet FastComments per Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ora posizioniamo il cursore sulla riga `101` - subito dopo il `</div>` - e incolliamo. Dovresti avere qualcosa del genere:

<div class="screenshot white-bg">
    <div class="title">Aggiungi il codice FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Aggiungi il codice FastComments" />
</div>

Ora possiamo salvare:

<div class="screenshot white-bg">
    <div class="title">Salva</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Salva" />
</div>

---