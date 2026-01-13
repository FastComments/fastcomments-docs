Sljedeće ćemo se pomaknuti dolje do retka `100`:

<div class="screenshot white-bg">
    <div class="title">Pomaknite se do retka 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Pomaknite se do retka 100" />
</div>

Now copy the following code snippet, which is designed **specifically for Shopify - do not use code snippets from other tutorials**:

[inline-code-attrs-start title = 'Shopify FastComments isječak'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada želimo postaviti kursor na `line 101` - odmah nakon `</div>` - i zalijepiti. Trebali biste dobiti nešto ovako:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Dodajte FastComments kod" />
</div>

Sada možemo spremiti:

<div class="screenshot white-bg">
    <div class="title">Spremi</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Spremi" />
</div>