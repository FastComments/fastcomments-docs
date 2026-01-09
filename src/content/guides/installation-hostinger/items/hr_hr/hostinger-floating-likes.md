FastComments također podržava Page Reacts (poznat i kao plutajući gumb Sviđa mi se) widget za Hostinger.

Možete ga vidjeti u akciji u donjem desnom kutu ove stranice!

### Napomena!

Ove upute su za Hostinger Site Builder. Ako koristite Hostinger *WordPress*, jednostavno kopirajte donji kod i dodajte ga na svoju WordPress stranicu
koristeći [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), što je besplatan i jednostavan dodatak za dodavanje malih fragmenata koda na vašu stranicu.

1. Prvo, kopirajte kod:

[inline-code-attrs-start title = 'Hostinger kod plutajućih lajkova'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Zatim, u Hostingeru otvorite Site Builder.
3. Idite na Website Settings u donjem lijevom kutu.
4. Odaberite Integrations.
5. Dodajte novi kod na *kraj* polja `Custom code`, i objavite svoju stranicu.
6. Widget nećete vidjeti u načinu pregleda, ali će se pojaviti na objavljenoj verziji stranice.