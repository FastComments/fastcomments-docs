Če ne morete namestiti [aplikacije iz Shopify App Store](https://apps.shopify.com/fastcomments), lahko FastComments vseeno dodate z urejanjem vaše teme. Ta pot je uporabna, kadar želite povezati FastComments najemnika, ki ga že imate, ali kadar vgrajujete komentarje na Shopify trgovino, kjer aplikacija ni možnost.

Namestitev prek aplikacije je podprta pot za večino trgovin. Uporabite to možnost le, če aplikacija ne ustreza.

### Korak 1: Onemogočite Shopifyjeve izvorne komentarje

V svojem Shopify skrbniškem vmesniku pojdite na **Blog posts > Manage blogs**, odprite vsak blog in v desnem panelu nastavite **Comments are disabled**. Shranite.

To prepreči, da bi se Shopifyjevi vgrajeni komentarji prikazovali skupaj s FastComments.

### Korak 2: Odprite predlogo teme za blog

V svojem Shopify skrbniškem vmesniku:

1. Pojdite na **Online Store > Themes**.
2. Pod svojo trenutno temo kliknite **Actions > Edit code**.
3. V brskalniku datotek na levi odprite **Sections** in kliknite `main-article.liquid`.

To je predloga, ki jo Shopify upodobi za posamezen članek na blogu.

### Korak 3: Prilepite FastComments izrezek

Pomaknite se približno na 100. vrstico datoteke `main-article.liquid`, tik za zaključnim `</div>` telesa članka. Prilepite naslednji izrezek:

[inline-code-attrs-start title = 'Shopify FastComments izrezek'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zamenjajte `"demo"` s svojim Tenant ID iz [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Kliknite **Save**.

### Korak 4: Avtorizirajte domeno svoje trgovine

Odprite objavo na blogu v svoji živi trgovini. Če namesto gradnika za komentarje vidite napako avtorizacije, mora FastComments vedeti, da vaša trgovina sme uporabljati ta najemnika. Glejte [Napake domen](/guide-installation-shopify.html#shopify-domain-errors).

### Dodajanje FastComments na druge strani

Isti izrezek deluje na kateri koli Liquid predlogi, vključno s stranimi izdelkov, stranimi po meri in domačo stranjo. Prilepite ga tam, kjer želite, da se komentarji prikažejo, in prilagodite `urlId`, če želite stabilen identifikator za vsako stran (na primer `urlId: "{{ product.id }}"` na predlogi izdelka).