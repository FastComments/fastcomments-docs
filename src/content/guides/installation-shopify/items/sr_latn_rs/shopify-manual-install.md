Ako ne možete da instalirate [Shopify App Store app](https://apps.shopify.com/fastcomments), i dalje možete dodati FastComments uređivanjem vaše teme. Ovaj postupak je koristan kada želite da povežete FastComments tenant koji već imate, ili kada ugrađujete na Shopify prodavnicu gde aplikacija nije opcija.

Instalacija putem aplikacije je podržani put za većinu prodavnica. Koristite ovaj način samo ako aplikacija nije odgovarajuća.

### Korak 1: Onemogućite ugrađene komentare Shopify-a

U vašem Shopify adminu, idite na **Blog posts > Manage blogs**, otvorite svaki blog i postavite **Komentari su onemogućeni** u desnom panelu. Sačuvajte.

Ovo sprečava da se Shopify-jevi ugrađeni komentari prikazuju zajedno sa FastComments.

### Korak 2: Otvorite šablon teme za blog

U vašem Shopify adminu:

1. Idite na **Online Store > Themes**.
2. Ispod vaše trenutne teme, kliknite **Actions > Edit code**.
3. U pregledaču fajlova levo, otvorite **Sections** i kliknite `main-article.liquid`.

Ovo je šablon koji Shopify prikazuje za jedan blog članak.

### Korak 3: Zalepite FastComments isječak

Skrolujte otprilike do linije 100 u `main-article.liquid`, odmah posle zatvarajućeg `</div>` tela članka. Zalepite sledeći isječak:

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

Zamenite `"demo"` svojim Tenant ID-jem sa [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Kliknite **Sačuvaj**.

### Korak 4: Autorizujte domen vaše prodavnice

Otvorite blog post na vašoj uživo prodavnici. Ako umesto widgeta za komentare vidite grešku autorizacije, FastComments treba da zna da je vašoj prodavnici dozvoljeno da koristi ovaj tenant. Pogledajte [Greške domena](/guide-installation-shopify.html#shopify-domain-errors).

### Dodavanje FastComments na druge stranice

Isti isječak radi u bilo kojem Liquid šablonu, uključujući stranice proizvoda, prilagođene stranice i početnu stranicu. Nalepite ga tamo gde želite da se komentari pojave i prilagodite `urlId` ako želite stabilan identifikator po stranici (na primer, `urlId: "{{ product.id }}"` u šablonu proizvoda).