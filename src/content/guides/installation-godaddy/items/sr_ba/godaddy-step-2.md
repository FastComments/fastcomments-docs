---
Sada kada ste dodali prilagođeni HTML blok, možemo dodati FastComments widget kod.

**Koristite sljedeći kod za Godaddy, ne kod iz drugih uputstava. Ovaj kod je specifičan za Godaddy.**

Kopirajte sljedeći kod:

[inline-code-attrs-start title = 'Isječak koda za Godaddy komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ovaj konkrektan isječak koda je dizajniran da bude kompatibilan s Godaddy-jem, i prikazivat će se samo na vašim blog postovima - ne na početnoj stranici.

Sada zalijepite kod u područje `Custom Code` pomenuto u `Step One`.

<div class="screenshot white-bg">
    <div class="title">Kopirajte i zalijepite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Kopirajte i zalijepite kod" />
</div>

Pritisnite Gotovo u gornjem desnom uglu:

<div class="screenshot white-bg">
    <div class="title">Kliknite Gotovo</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Kliknite Gotovo" />
</div>

To je sve za drugi korak!

---