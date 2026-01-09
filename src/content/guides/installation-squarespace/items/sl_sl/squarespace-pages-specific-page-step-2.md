Zdaj lahko kopiramo naslednji odsek kode. Uporabite gumb za kopiranje, ki se prikaže v zgornjem desnem kotu odseka.

V kodi lahko konfigurirate nekaj stvari, glejte vrstice 4 do 7.

[inline-code-attrs-start title = 'Koda za posamezno stran Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ID vašega računa

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Izgledati bi moralo takole:

<div class="screenshot white-bg">
    <div class="title">Prilepi in shrani</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Prilepi in shrani" />
</div>

Zdaj kliknite Shrani v zgornjem desnem kotu.

Upoštevajte, da možnost `Preview in Safe Mode` ne bo delovala, vendar se bo widget prikazal, ko obiščete svojo stran.

Če imate težave, preverite, da pri dnu ne piše `"tenantId": "demo"`. Moralo bi prikazati vaš tenant id, če ste prijavljeni. Če ne, se obrnite na podporo.