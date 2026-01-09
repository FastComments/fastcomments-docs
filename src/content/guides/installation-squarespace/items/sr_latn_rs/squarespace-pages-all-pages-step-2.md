Sada možemo kopirati sledeći isječak koda. Koristite dugme za kopiranje koje se pojavljuje u gornjem desnom uglu isječka.

Postoji nekoliko stvari koje možete konfigurisati u kodu, pogledajte linije 4 do 7.

[inline-code-attrs-start title = 'Squarespace kod komentara za sve stranice'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // vaš ID naloga

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

...zatim zalepite u polje za kod i kliknite sačuvaj. Trebalo bi da izgleda ovako, sa kodom u `FOOTER` bloku:

<div class="screenshot white-bg">
    <div class="title">Zalepi i sačuvaj</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Zalepi i sačuvaj" />
</div>

Ako imate problema, uverite se da pri dnu ne piše `"tenantId": "demo"`. Treba da prikaže vaš tenant id ako ste prijavljeni. Ako ne, obratite se podršci.