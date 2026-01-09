Sada kada ste dodali prilagođeni HTML blok, možemo dodati FastComments widget kod.

**Koristite sledeći kod za Godaddy, ne kod iz drugih tutorijala. Ovaj kod je Godaddy-specifičan.**

Kopirajte sledeći kod:

[inline-code-attrs-start title = 'Isječak koda komentara za Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ovaj konkretan isječak koda je dizajniran da bude kompatibilan sa Godaddy i takođe će se prikazivati samo na vašim blog postovima - ne na naslovnoj strani.

Sada nalepite kod u oblast `Custom Code` pomenutu u `Step One`.

<div class="screenshot white-bg">
    <div class="title">Kopirajte i nalepite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Kopirajte i nalepite kod" />
</div>

Pritisnite Done u gornjem desnom uglu:

<div class="screenshot white-bg">
    <div class="title">Kliknite Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Kliknite Done" />
</div>

To je sve za drugi korak!

---