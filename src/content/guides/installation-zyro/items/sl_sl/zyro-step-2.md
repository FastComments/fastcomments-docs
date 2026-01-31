Zdaj dodajmo kodo našega widgeta.

Kopirajte spodnjo kodo. Poskrbite, da ste prijavljeni na [fastcomments.com](https://fastcomments.com) 
in po potrebi znova naložite to stran, da se koda samodejno izpolni z informacijami o vašem računu; sicer bo prikazana demo koda.

Zdaj kopirajmo kodo:

[inline-code-attrs-start title = 'Koda komentarjev Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Zdaj se vrnimo v urejevalnik spletne strani in kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Vnesite kodo</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Vnesite kodo" />
</div>

### Opomba!

Pomembno je, da uporabite zgornjo kodo in ne odlomkov kode iz druge dokumentacije, saj je ta košček kode ustvarjen posebej
za Zyro.

Zdaj bi morali imeti nekaj takega, kar se zdi prazno. To je pričakovano. Premaknite miško nad območje
kjer bi moral biti widget:

<div class="screenshot white-bg">
    <div class="title">Koda widgeta dodana</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Koda widgeta dodana" />
</div>

Zdaj povlecite widget na želeno velikost; videli boste, da se prikaže:

<div class="screenshot white-bg">
    <div class="title">Prilagodite velikost</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Prilagodite velikost" />
</div>

...in sedaj predogledajte in shranite!

---