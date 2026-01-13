Zdaj dodajmo kodo našega widgeta.

Kopirajte spodnjo kodo. Prepričajte se, da ste prijavljeni na [fastcomments.com](https://fastcomments.com) 
in po potrebi ponovno naložite to stran, da se koda samodejno predpopolni z informacijami vašega računa, sicer se bo prikazala demo koda.

Zdaj kopirajmo kodo:

[inline-code-attrs-start title = 'Koda komentarjev Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Zdaj se vrnemo v urejevalnik strani in kliknemo `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Vnesi kodo</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Vnesi kodo" />
</div>

### Opomba!

Pomembno je, da uporabite zgornjo kodo in ne izrezkov iz druge dokumentacije, saj je ta del kode posebej pripravljen za Hostinger.

Sedaj bi morali videti nekaj takšnega, kar se zdi prazno. To je pričakovano. Premaknite miško nad območje, kjer naj bi se pojavil widget:

<div class="screenshot white-bg">
    <div class="title">Dodani widget</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodani widget" />
</div>

Zdaj povlecite widget na želeno velikost; prikazal se bo:

<div class="screenshot white-bg">
    <div class="title">Prilagodi velikost</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Prilagodi velikost" />
</div>

...in zdaj predoglejte in shranite!