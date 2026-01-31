Als Nächstes scrollen wir zur Zeile `100`:

<div class="screenshot white-bg">
    <div class="title">Zur Zeile 100 scrollen</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Zur Zeile 100 scrollen" />
</div>

Kopieren Sie nun den folgenden Codeausschnitt, der **speziell für Shopify erstellt wurde - verwenden Sie keine Codeausschnitte aus anderen Tutorials**:

[inline-code-attrs-start title = 'Shopify FastComments Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Jetzt setzen wir unseren Cursor auf `line 101` - direkt nach dem `</div>` - und fügen ein. Sie sollten so etwas haben:

<div class="screenshot white-bg">
    <div class="title">FastComments-Code hinzufügen</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="FastComments-Code hinzufügen" />
</div>

Nun können wir speichern:

<div class="screenshot white-bg">
    <div class="title">Speichern</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Speichern" />
</div>