Sada kada smo u uređivaču šablona, moramo odlučiti gdje želimo da prikažemo komentare ili ćaskanje uživo.

U ovom primjeru dodaćemo ga direktno ispod videa. Pređite mišem preko elementa koji želite dodati widget na kraj, i kliknite `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Dodaj element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Dodaj element" />
</div>

Odaberite `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Odaberite CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Odaberite CUSTOM JS/HTML" />
</div>

Sada otvorimo uređivač koda gdje ćemo nalijepiti naš kod.

ClickFunnels može biti malo zbunjujući u sljedećem koraku.

Važno je da *NE* odaberete `Code` kada pređete mišem preko novog elementa. Umjesto toga, odaberite `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Odaberite SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Odaberite SETTINGS" />
</div>

Sada na desnoj strani možemo kliknuti `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Kliknite Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Kliknite Open Code Editor" />
</div>

Vidjećete da će se otvoriti veliki kvadrat. Tu možemo nalijepiti naš kod. Kopirajte sljedeći isječak (koristite dugme za kopiranje u gornjem desnom uglu):

[inline-code-attrs-start title = 'Isječak koda za ClickFunnels Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // neki provajderi mijenjaju isječak koda da bude asinhron
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

Ovaj isječak koda je za naš proizvod Streaming Chat, koji se dobro slaže sa video-sadržajem. Ako želite umjesto toga isječak koda za Live Commenting widget, koji najbolje pristaje za obične stranice ili blog postove, on se nalazi na kraju ovog vodiča.

Kada nalijepimo isječak koda u prozor, trebalo bi da izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Nalijepi kod</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Nalijepi kod" />
</div>

Sada samo moramo zatvoriti okvir:

<div class="screenshot white-bg">
    <div class="title">Zatvori</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Zatvori" />
</div>

Sada možete pregledati svoje izmjene! Slobodno pomjerajte widget i provjerite gdje vam najviše odgovara.

<div class="screenshot white-bg">
    <div class="title">Pregled</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Pregled" />
</div>

Uspješno! Ne zaboravite da testirate na mobilnim uređajima!

<div class="screenshot white-bg">
    <div class="title">Uspjeh!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Uspjeh!" />
</div>