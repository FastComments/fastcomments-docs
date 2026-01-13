---
FastComments takođe podržava widget Page Reacts (poznat i kao plutajuće dugme za lajk) za Zyro.

Možete ga videti u akciji u donjem desnom uglu ove stranice!

1. Prvo, preuzmite kod:

[inline-code-attrs-start title = 'Zyro kod za plutajuće lajkove'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Zatim, u Zyro, otvorite uređivač sajta.
3. Idite na Podešavanja sajta u donjem levom uglu.
4. Izaberite Integracije.
5. Dodajte novi kod na *kraj* polja `Custom code`, i objavite vaš sajt.
6. Nećete videti widget u režimu pregleda, ali pojaviće se na objavljenoj verziji sajta.

---