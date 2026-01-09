Sada kada smo u uređivaču predložaka, moramo odlučiti gdje želimo prikazati komentare ili razgovor uživo.

U ovom ćemo primjeru dodati widget izravno ispod videa. Zadržite pokazivač iznad elementa da biste dodali widget na kraj elementa, pa kliknite `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Dodaj element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Dodaj element" />
</div>

Odaberite `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Odaberite CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Odaberite CUSTOM JS/HTML" />
</div>

Sada otvorimo uređivač koda u koji ćemo zalijepiti naš kod.

ClickFunnels je pomalo zbunjujuć u sljedećem koraku.

Važno je da *NE* odaberete `Code` kada zadržite pokazivač iznad novog elementa. Umjesto toga, odaberite `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Odaberite SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Odaberite SETTINGS" />
</div>

Sada na desnoj strani možemo kliknuti `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Kliknite Otvori uređivač koda</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Kliknite Otvori uređivač koda" />
</div>

Vidjet ćete da se otvori veliki kvadrat. Ovdje možemo zalijepiti naš kod. Kopirajte sljedeći isječak (upotrijebite gumb za kopiranje u gornjem desnom kutu):

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
            // neki pružatelji usluga mijenjaju isječak koda da bude asinhron
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

Ovaj isječak koda je za naš proizvod Streaming Chat, koji dobro pristaje uz videozapise. Ako želite isječak koda za widget Live Commenting, koji je prikladniji za obične stranice ili blog postove, on se nalazi na kraju ovog vodiča.

Kada zalijepimo isječak koda u prozor, trebao bi izgledati ovako:

<div class="screenshot white-bg">
    <div class="title">Zalijepi kod</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Zalijepi kod" />
</div>

Sada samo moramo zatvoriti okvir:

<div class="screenshot white-bg">
    <div class="title">Zatvori</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Zatvori" />
</div>

Sada možete pregledati svoje promjene! Slobodno pomaknite widget i vidite gdje vam najviše odgovara.

<div class="screenshot white-bg">
    <div class="title">Pregled</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Pregled" />
</div>

Uspješno! Ne zaboravite testirati na mobilnim uređajima!

<div class="screenshot white-bg">
    <div class="title">Uspješno!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Uspješno!" />
</div>