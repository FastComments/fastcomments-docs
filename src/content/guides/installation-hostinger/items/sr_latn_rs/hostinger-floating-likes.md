FastComments takođe podržava Page Reacts (poznat i kao plutajuće dugme „Sviđa mi se“) vidžet za Hostinger.

Možete ga videti u donjem desnom uglu ove stranice!

### Napomena!

Ova uputstva su za Hostinger Site Builder. Ako koristite Hostinger *WordPress*, jednostavno kopirajte kod ispod i dodajte ga na vaš WordPress sajt koristeći [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), koji je besplatan i jednostavan dodatak za dodavanje malih kodnih isječaka na vaš sajt.

1. Prvo, kopirajte kod:

[inline-code-attrs-start title = 'Hostinger kod za plutajuće dugme "Sviđa mi se"'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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
3. Idite na Podešavanja sajta u donjem levom uglu.
4. Izaberite Integracije.
5. Dodajte novi kod na *kraj* polja `Custom code`, i objavite svoj sajt.
6. Nećete videti vidžet u režimu pregleda, ali će se pojaviti u objavljenoj verziji sajta.

---