If you can't install the [Shopify App Store app](https://apps.shopify.com/fastcomments), you can still add FastComments by editing your theme. This path is useful when you want to wire up a FastComments tenant you already own, or when you're embedding on a Shopify storefront where the app isn't an option.

The app-based install is the supported path for most stores. Reach for this only if the app doesn't fit.

### Korak 1: Onemogočite vgrajene komentarje Shopify

V svojem Shopify skrbniškem vmesniku pojdite na **Objave v blogu > Upravljaj bloge**, odprite vsak blog in v desnem panelu nastavite **Komentarji so onemogočeni**. Shrani.

S tem preprečite, da bi se Shopifyjevi vgrajeni komentarji prikazovali skupaj s FastComments.

### Korak 2: Odprite predlogo teme za blog

V svojem Shopify skrbniškem vmesniku:

1. Pojdite na **Spletna trgovina > Teme**.
2. Pod vašo trenutno temo kliknite **Dejanja > Uredi kodo**.
3. V brskalniku datotek na levi odprite **Sekcije** in kliknite `main-article.liquid`.

To je predloga, ki jo Shopify uporablja za prikaz posameznega blog članka.

### Korak 3: Prilepite FastComments odsek

Pomaknite se približno do vrstice 100 v `main-article.liquid`, takoj za zapiralnim `</div>` telesa članka. Prilepite naslednji odsek:

[inline-code-attrs-start title = 'FastComments vstavka za Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zamenjajte `"demo"` z vašim Tenant ID iz [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Kliknite **Shrani**.

### Korak 4: Avtorizirajte domeno vaše trgovine

Odprite objavo v blogu na vaši živi trgovini. Če namesto pripomočka za komentarje vidite napako pri avtorizaciji, FastComments potrebuje vedeti, da je vaša trgovina pooblaščena za uporabo tega najemnika. Oglejte si [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors).

### Dodajanje FastComments na druge strani

Isti vstavki delujejo v kateri koli Liquid predlogi, vključno s stranmi izdelkov, prilagojenimi stranmi in domačo stranjo. Prilepite ga tam, kjer želite, da se prikažejo komentarji, in prilagodite `urlId`, če želite stabilen identifikator za posamezno stran (na primer, `urlId: "{{ product.id }}"` v predlogi izdelka).

---