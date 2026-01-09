Zdaj, ko smo v urejevalniku predlog, moramo odločiti, kje želimo prikazati komentarje ali klepet v živo.

V tem primeru ga bomo dodali neposredno pod videoposnetek. Pokažite miško nad elementom, da dodate pripomoček na konec, in kliknite `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Dodaj element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Dodaj element" />
</div>

Izberite `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Izberite CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Izberite CUSTOM JS/HTML" />
</div>

Zdaj odprite urejevalnik kode, kamor bomo prilepili našo kodo.

ClickFunnels je pri naslednjem koraku nekoliko zmeden.

Pomembno je, da *NE* izberete `Code` ko pokažete miško nad novim elementom. Namesto tega izberite `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Izberite SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Izberite SETTINGS" />
</div>

Zdaj na desni strani lahko kliknemo `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Kliknite Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Kliknite Open Code Editor" />
</div>

Videli boste odprto veliko polje. Tukaj lahko prilepimo našo kodo. Kopirajte naslednji odlomek (uporabite gumb za kopiranje v zgornjem desnem kotu):

[inline-code-attrs-start title = 'Primer kode za ClickFunnels Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Ta odlomek kode je za naš izdelek Streaming Chat, ki se dobro ujema z videoposnetki. Če želite namesto tega kodo pripomočka Live Commenting, ki
se najbolj prilega običajnim stranem ali blog objavam, je na koncu tega vodiča.

Ko prilepimo odlomek kode v okno, bi moralo izgledati tako:

<div class="screenshot white-bg">
    <div class="title">Prilepi kodo</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Prilepi kodo" />
</div>

Zdaj moramo le zapreti okno:

<div class="screenshot white-bg">
    <div class="title">Zapri</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Zapri" />
</div>

Zdaj lahko predogledate svoje spremembe! Prosto premaknite pripomoček in preverite, kje vam je najbolj všeč.

<div class="screenshot white-bg">
    <div class="title">Predogled</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Predogled" />
</div>

Uspešno! Ne pozabite preizkusiti na mobilnih napravah!

<div class="screenshot white-bg">
    <div class="title">Uspešno!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Uspešno!" />
</div>