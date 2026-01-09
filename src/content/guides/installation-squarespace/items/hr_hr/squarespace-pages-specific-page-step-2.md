Sada možemo kopirati sljedeći isječak koda. Upotrijebite gumb za kopiranje koji se pojavljuje u gornjem desnom kutu isječka.

Postoji nekoliko stvari koje možete konfigurirati u kodu; pogledajte redove 4 do 7.

[inline-code-attrs-start title = 'Squarespace kod za jednu stranicu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // vaš ID računa

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Trebao bi izgledati ovako:

<div class="screenshot white-bg">
    <div class="title">Zalijepite i spremite</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Zalijepite i spremite" />
</div>

Sada kliknite Spremi u gornjem desnom kutu.

Imajte na umu da opcija `Preview in Safe Mode` neće raditi, ali widget će se pojaviti kada posjetite svoju stranicu.

Ako imate problema, provjerite da pri dnu ne piše `"tenantId": "demo"`. Trebalo bi prikazati vaš tenant id ako ste prijavljeni. Ako nije, obratite se podršci.

---