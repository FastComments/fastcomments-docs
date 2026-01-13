---
Sada možemo kopirati sljedeći isječak koda. Koristite dugme za kopiranje koje se pojavljuje u gornjem desnom kutu isječka.

Postoji nekoliko stvari koje možete konfigurisati u kodu, pogledajte linije 4 do 7.

[inline-code-attrs-start title = 'Squarespace Kod za komentare na svim stranicama'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...then paste in the code area and click save. It should look like this, with the code in the `FOOTER` block:

<div class="screenshot white-bg">
    <div class="title">Zalijepi i sačuvaj</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Zalijepi i sačuvaj" />
</div>

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.

---