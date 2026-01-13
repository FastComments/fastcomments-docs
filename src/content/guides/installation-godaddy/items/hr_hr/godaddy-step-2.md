---
Sada kada ste dodali prilagođeni HTML blok, možemo dodati kôd widgeta FastComments.

**Upotrijebite sljedeći kôd za Godaddy, ne kôd iz drugih vodiča. Ovaj kôd je specifičan za Godaddy.**

Kopirajte sljedeći kôd:

[inline-code-attrs-start title = 'Isječak kôda komentara za Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ovaj specifični isječak kôda dizajniran je da bude kompatibilan s Godaddyjem i prikazivat će se samo na vašim objavama na blogu — ne na početnoj stranici.

Sada zalijepite kôd u područje `Custom Code` spomenuto u `Step One`.

<div class="screenshot white-bg">
    <div class="title">Kopirajte i zalijepite kôd</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Kopirajte i zalijepite kôd" />
</div>

Pritisnite Done u gornjem desnom kutu:

<div class="screenshot white-bg">
    <div class="title">Kliknite Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Kliknite Done" />
</div>

To je to za drugi korak!

---