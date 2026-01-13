Sledeće ćemo se spustiti do linije `100`:

<div class="screenshot white-bg">
    <div class="title">Spustite se do linije 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Spustite se do linije 100" />
</div>

Sada kopirajte sljedeći isječak koda, koji je dizajniran **specijalno za Shopify - ne koristite isječke koda iz drugih tutorijala**:

[inline-code-attrs-start title = 'FastComments isječak za Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada želimo da postavimo kursor na `line 101` - odmah nakon `</div>` - i zalijepimo. Trebalo bi da imate nešto ovako:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Dodajte FastComments kod" />
</div>

Sada možemo sačuvati:

<div class="screenshot white-bg">
    <div class="title">Sačuvaj</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Sačuvaj" />
</div>