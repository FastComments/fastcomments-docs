Nu hvor du har tilføjet en tilpasset HTML-blok, kan vi tilføje FastComments-widget-koden.

**Brug følgende kode til Godaddy, ikke koden fra andre vejledninger. Denne kode er specifik for Godaddy.**

Copy the following code:

[inline-code-attrs-start title = 'Godaddy-kommentarer kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Dette specifikke kodeudsnit er designet til at være kompatibelt med Godaddy, og vil kun blive vist på dine blogindlæg - ikke på startsiden.

Indsæt nu koden i området `Custom Code`, som er nævnt i `Step One`.

<div class="screenshot white-bg">
    <div class="title">Kopier og indsæt koden</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Kopier og indsæt koden" />
</div>

Klik på 'Done' øverst til højre:

<div class="screenshot white-bg">
    <div class="title">Klik på 'Done'</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Klik på 'Done'" />
</div>

Det var det for trin to!

---