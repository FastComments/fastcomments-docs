Sada možemo kopirati sljedeći isječak koda. Koristite gumb za kopiranje koji se pojavljuje u gornjem desnom kutu isječka.

Postoji nekoliko stvari koje možete konfigurirati u kodu, pogledajte retke 4 do 7.

[inline-code-attrs-start title = 'Squarespace kod komentara za sve stranice'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // vaš ID računa

        function tryLoad() {
            // pokušaj učitavanja za različite rasporede
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...zatim zalijepite u područje koda i kliknite Spremi. Trebalo bi izgledati ovako, s kodom u bloku `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Zalijepi i spremi</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Zalijepi i spremi" />
</div>

Ako imate poteškoća, provjerite da pri dnu ne piše `"tenantId": "demo"`. Trebalo bi prikazati vaš tenant id ako ste prijavljeni. Ako nije, obratite se podršci.