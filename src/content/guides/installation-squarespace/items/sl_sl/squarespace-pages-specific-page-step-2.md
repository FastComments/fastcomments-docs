Zdaj lahko kopiramo naslednji odsek kode. Uporabite gumb za kopiranje, ki se prikaže v zgornjem desnem kotu odseka.

V kodi lahko nastavite nekaj stvari, oglejte si vrstice 4 do 7.

[inline-code-attrs-start title = 'Squarespace koda za eno stran'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // vaš ID računa
    }];
</script>
[inline-code-end]

Moral bi izgledati takole:

<div class="screenshot white-bg">
    <div class="title">Prilepi in Shrani</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Prilepi in Shrani" />
</div>

Zdaj kliknite Shrani v zgornjem desnem kotu.

Upoštevajte, da možnost `Preview in Safe Mode` ne bo delovala, vendar se bo widget prikazal, ko obiščete svoje spletno mesto.

Če imate težave, preverite pri dnu, da ne piše `"tenantId": "demo"`. Moral bi prikazati vaš tenant id, če ste prijavljeni. Če ne, se obrnite na podporo.