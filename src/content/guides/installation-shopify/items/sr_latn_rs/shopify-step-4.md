Sledeće ćemo se pomeriti na liniju `100`:

<div class="screenshot white-bg">
    <div class="title">Skroluj do linije 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Skroluj do linije 100" />
</div>

Sada kopirajte sledeći isječak koda, koji je dizajniran **posebno za Shopify - nemojte koristiti isječke koda iz drugih tutorijala**:

[inline-code-attrs-start title = 'Shopify FastComments isječak'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada želimo da postavimo kursor na `line 101` - odmah posle `</div>` - i nalepimo. Trebalo bi da imate nešto poput ovog:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Dodajte FastComments kod" />
</div>

Sada možemo sačuvati:

<div class="screenshot white-bg">
    <div class="title">Sačuvaj</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Sačuvaj" />
</div>

---