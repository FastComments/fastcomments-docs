Ako ne možete instalirati [Shopify App Store aplikaciju](https://apps.shopify.com/fastcomments), i dalje možete dodati FastComments uređivanjem svoje teme. Ovaj način je koristan kada želite povezati FastComments tenant koji već imate, ili kada ugrađujete na Shopify trgovinu gdje aplikacija nije opcija.

Instalacija putem aplikacije je podržani put za većinu trgovina. Koristite ovaj način samo ako aplikacija nije prikladna.

### Korak 1: Onemogućite izvorne komentare Shopifyja

U svom Shopify administratorskom sučelju idite na **Blog posts > Manage blogs**, otvorite svaki blog i u desnom panelu postavite **Comments are disabled**. Kliknite **Save**.

Ovo sprječava da Shopifyjevi ugrađeni komentari budu prikazani uz FastComments.

### Korak 2: Otvorite predložak teme bloga

U Shopify administraciji:

1. Idite na **Online Store > Themes**.
2. Ispod trenutne teme kliknite **Actions > Edit code**.
3. U pregledniku datoteka s lijeve strane, otvorite **Sections** i kliknite `main-article.liquid`.

Ovo je predložak koji Shopify prikazuje za pojedinačni blog članak.

### Korak 3: Zalijepite FastComments isječak

Pomaknite se otprilike do retka 100 u `main-article.liquid`, odmah nakon zatvarajućeg `</div>` tijela članka. Zalijepite sljedeći isječak:

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

Zamijenite `"demo"` vlastitim Tenant ID-jem s [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Kliknite **Save**.

### Korak 4: Autorizirajte domenu vaše trgovine

Otvorite post s bloga na vašoj javnoj trgovini. Ako umjesto widgeta za komentare vidite grešku autorizacije, FastComments treba znati da je vašoj trgovini dopušteno koristiti ovaj tenant. Pogledajte [Pogreške domene](/guide-installation-shopify.html#shopify-domain-errors).

### Dodavanje FastComments na druge stranice

Isti isječak radi u bilo kojem Liquid predlošku, uključujući stranice proizvoda, prilagođene stranice i početnu stranicu. Zalijepite ga tamo gdje želite da se komentari prikazuju i prilagodite `urlId` ako želite stabilan identifikator po stranici (na primjer, `urlId: "{{ product.id }}"` na predlošku proizvoda).