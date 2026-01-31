Sada možemo kopirati sledeći isječak koda. Koristite dugme za kopiranje koje se pojavljuje u gornjem desnom uglu isječka.

Postoji nekoliko stvari koje možete konfigurisati u kodu, pogledajte linije 4 do 7.

[inline-code-attrs-start title = 'Squarespace kod za jednu stranicu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // vaš ID naloga
    }];
</script>
[inline-code-end]

Treba da izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Paste and Save</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Paste and Save" />
</div>

Sada kliknite na Sačuvaj u gornjem desnom uglu.

Imajte na umu da opcija `Preview in Safe Mode` neće raditi, ali widget će se pojaviti kada posetite svoj sajt.

Ako imate problema, proverite pri dnu da ne piše `"tenantId": "demo"`. Trebalo bi da prikazuje vaš tenant id ako ste prijavljeni. Ako ne, obratite se podršci.