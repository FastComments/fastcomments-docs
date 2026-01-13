Nu je een aangepast HTML-blok hebt toegevoegd, kunnen we de FastComments-widgetcode toevoegen.

**Gebruik de volgende code voor Godaddy, niet de code uit andere handleidingen. Deze code is specifiek voor Godaddy.**

Kopieer de volgende code:

[inline-code-attrs-start title = 'Godaddy-opmerkingen codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Dit specifieke codefragment is ontworpen om compatibel te zijn met Godaddy, en wordt alleen weergegeven op je blogposts - niet op de homepage.

Plak nu de code in het gebied `Custom Code` dat genoemd wordt in `Step One`.

<div class="screenshot white-bg">
    <div class="title">Kopieer en plak de code</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Kopieer en plak de code" />
</div>

Klik op Done rechtsboven:

<div class="screenshot white-bg">
    <div class="title">Klik op Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Klik op Done" />
</div>

Dat is alles voor stap twee!

---