Zdaj, ko ste dodali blok po meri HTML, lahko dodamo kodo pripomočka FastComments.

**Uporabite naslednjo kodo za Godaddy, ne kode iz drugih vodičev. Ta koda je specifična za Godaddy.**

Kopirajte naslednjo kodo:

[inline-code-attrs-start title = 'Odlomek kode za komentarje Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Ta specifični odlomek kode je zasnovan za združljivost z Godaddy in se bo prikazal le na vaših objavah bloga – ne na domači strani.

Zdaj prilepite kodo v območje `Custom Code`, omenjeno v `Prvem koraku`.

<div class="screenshot white-bg">
    <div class="title">Kopirajte in prilepite kodo</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Kopirajte in prilepite kodo" />
</div>

Kliknite Done v zgornjem desnem kotu:

<div class="screenshot white-bg">
    <div class="title">Kliknite Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Kliknite Done" />
</div>

To je vse za drugi korak!