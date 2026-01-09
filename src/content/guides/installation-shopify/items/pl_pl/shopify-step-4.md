---
Następnie przewiniemy w dół do linii `100`:

<div class="screenshot white-bg">
    <div class="title">Przewiń do linii 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Przewiń do linii 100" />
</div>

Teraz skopiuj następujący fragment kodu, który został stworzony **specjalnie dla Shopify - nie używaj fragmentów kodu z innych samouczków**:

[inline-code-attrs-start title = 'Fragment FastComments dla Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Teraz ustaw kursor na `linii 101` - tuż za `</div>` - i wklej. Powinieneś mieć coś takiego:

<div class="screenshot white-bg">
    <div class="title">Dodaj kod FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Dodaj kod FastComments" />
</div>

Teraz możemy zapisać:

<div class="screenshot white-bg">
    <div class="title">Zapisz</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Zapisz" />
</div>

---