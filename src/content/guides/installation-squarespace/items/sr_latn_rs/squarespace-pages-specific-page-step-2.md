---
Sada možemo kopirati sledeći isječak koda. Koristite dugme za kopiranje koje se pojavljuje u gornjem desnom uglu isječka.

Postoji nekoliko stvari koje možete konfigurisati u kodu, pogledajte linije 4 do 7.

[inline-code-attrs-start title = 'Squarespace kod za jednu stranicu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // vaš ID naloga

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Treba da izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Nalepi i sačuvaj</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Nalepi i sačuvaj" />
</div>

Sada kliknite na Sačuvaj u gornjem desnom uglu.

Imajte na umu da opcija `Preview in Safe Mode` neće raditi, ali će widget biti prikazan kada posetite vaš sajt.

Ako imate probleme, proverite da li pri dnu nije navedeno `"tenantId": "demo"`. Trebalo bi da prikaže vaš tenant id ako ste prijavljeni. Ako nije, obratite se podršci.

---