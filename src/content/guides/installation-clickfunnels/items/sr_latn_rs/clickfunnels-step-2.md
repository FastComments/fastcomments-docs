Sada kada smo u uređivaču šablona, moramo odlučiti gde želimo da prikažemo komentare ili live chat.

U ovom primeru dodaćemo ga odmah ispod videa. Pređite mišem preko elementa da biste dodali widget na kraj, i kliknite `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Dodaj element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Dodaj element" />
</div>

Izaberite `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Izaberite CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Izaberite CUSTOM JS/HTML" />
</div>

Sada ćemo otvoriti uređivač koda gde ćemo nalepiti naš kod.

ClickFunnels je pomalo zbunjujući u sledećem koraku.

Važno je da *NE* izaberete `Code` kada pređete mišem preko novog elementa. Umesto toga, izaberite `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Izaberite SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Izaberite SETTINGS" />
</div>

Sada, na desnoj strani možemo kliknuti `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Kliknite Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Kliknite Open Code Editor" />
</div>

Videćete veliki kvadrat koji se otvara. Ovde možemo nalepiti naš kod. Kopirajte sledeći isječak koda (koristite dugme za kopiranje u gornjem desnom uglu):

[inline-code-attrs-start title = 'ClickFunnels kod za Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // neki provajderi menjaju snippet koda da bude asinhron
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

Ovaj isječak koda je za naš Streaming Chat proizvod, koji se dobro slaže sa video sadržajem. Ako želite umesto toga kod za Live Commenting widget, koji najbolje funkcioniše sa običnim stranicama ili blog postovima, on se nalazi na kraju ovog tutorijala.

Kada nalepite isječak koda u prozor, trebalo bi da izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Nalepi kod</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Nalepi kod" />
</div>

Sada samo treba da zatvorimo prozor:

<div class="screenshot white-bg">
    <div class="title">Zatvori</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Zatvori" />
</div>

Sada možete pregledati svoje izmene! Slobodno pomerajte widget i probajte gde vam najviše odgovara.

<div class="screenshot white-bg">
    <div class="title">Pregled</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Pregled" />
</div>

Uspešno! Ne zaboravite da testirate mobilne uređaje!

<div class="screenshot white-bg">
    <div class="title">Uspeh!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Uspeh!" />
</div>