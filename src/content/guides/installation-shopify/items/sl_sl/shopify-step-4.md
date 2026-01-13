Nato se bomo pomaknili navzdol do vrstice `100`:

<div class="screenshot white-bg">
    <div class="title">Pomaknite se do vrstice 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Pomaknite se do vrstice 100" />
</div>

Zdaj kopirajte naslednji izrezek kode, ki je zasnovan **posebej za Shopify - ne uporabljajte izrezkov kode iz drugih vodičev**:

[inline-code-attrs-start title = 'Izrezek FastComments za Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zdaj postavite kazalec na `line 101` - right after the `</div>` - and paste. You should have something like this:

<div class="screenshot white-bg">
    <div class="title">Dodajte kodo FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Dodajte kodo FastComments" />
</div>

Zdaj lahko shranimo:

<div class="screenshot white-bg">
    <div class="title">Shrani</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Shrani" />
</div>

---