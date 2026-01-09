Als Nächstes scrollen wir zu Zeile `100`:

<div class="screenshot white-bg">
    <div class="title">Zu Zeile 100 scrollen</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Zu Zeile 100 scrollen" />
</div>

Kopieren Sie nun den folgenden Code-Snippet, der **speziell für Shopify entwickelt wurde - verwenden Sie keine Code-Snippets aus anderen Tutorials**:

[inline-code-attrs-start title = 'Shopify FastComments-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Nun setzen wir den Cursor auf `line 101` - direkt nach dem `</div>` - und fügen ein. Sie sollten so etwas wie das Folgende haben:

<div class="screenshot white-bg">
    <div class="title">FastComments-Code hinzufügen</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="FastComments-Code hinzufügen" />
</div>

Jetzt können wir speichern:

<div class="screenshot white-bg">
    <div class="title">Speichern</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Speichern" />
</div>

---