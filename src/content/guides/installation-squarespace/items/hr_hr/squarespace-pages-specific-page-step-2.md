Sada možemo kopirati sljedeći isječak koda. Koristite gumb za kopiranje koji se pojavljuje u gornjem desnom kutu isječka.

Postoji nekoliko stvari koje možete konfigurirati u kodu, pogledajte linije 4 do 7.

[inline-code-attrs-start title = 'Squarespace kod za pojedinačnu stranicu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Trebalo bi izgledati ovako:

<div class="screenshot white-bg">
    <div class="title">Zalijepi i spremi</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Zalijepi i spremi" />
</div>

Sada kliknite spremi u gornjem desnom kutu.

Imajte na umu da opcija `Preview in Safe Mode` neće raditi, ali widget će se pojaviti kada posjetite svoju stranicu.

Ako imate problema, provjerite pri dnu da ne piše `"tenantId": "demo"`. Trebalo bi prikazati vaš tenant id ako ste prijavljeni. Ako ne, obratite se podršci.