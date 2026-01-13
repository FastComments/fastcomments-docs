---
FastComments također podržava widget Page Reacts (poznat i kao plutajuće dugme 'Sviđa mi se') za Zyro.

Možete ga vidjeti u akciji u donjem desnom kutu ove stranice!

1. Prvo, preuzmite kod:

[inline-code-attrs-start title = 'Zyro kod za plutajuće sviđanja'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Zatim, u Zyro, otvorite graditelj stranice.
3. Idite na Postavke web-stranice u donjem lijevom kutu.
4. Odaberite Integracije.
5. Dodajte novi kod na *kraj* polja `Custom code`, i objavite vašu stranicu.
6. Widget se neće pojaviti u načinu pregleda, ali će se pojaviti na objavljenoj verziji stranice.

---