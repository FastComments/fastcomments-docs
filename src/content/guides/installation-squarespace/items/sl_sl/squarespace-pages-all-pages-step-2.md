Zdaj lahko kopiramo naslednji delček kode. Uporabite gumb za kopiranje, ki se prikaže v zgornjem desnem kotu delčka.

V kodi lahko nastavite nekaj stvari, glejte vrstice 4 do 7.

[inline-code-attrs-start title = 'Koda za komentarje na vseh straneh Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // your account id

        function tryLoad() {
            // try to load for different layouts
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

...nato prilepite kodo v območje za kodo in kliknite shrani. Videti mora biti takole, s kodo v bloku `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Prilepi in shrani</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Prilepi in shrani" />
</div>

Če imate težave, preverite, ali blizu dna ni napisano `"tenantId": "demo"`. Če ste prijavljeni, bi moral prikazati vaš ID najemnika. Če ne, se obrnite na podporo.