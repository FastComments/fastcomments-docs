Nu hvor vi er i skabeloneditoren, skal vi beslutte, hvor vi vil vise kommentarerne eller live chatten.

I dette eksempel tilføjer vi den direkte under videoen. Hold musen over det element, hvor du vil føje widgetten til, og klik på `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Tilføj element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Tilføj element" />
</div>

Vælg `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Vælg CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Vælg CUSTOM JS/HTML" />
</div>

Lad os nu åbne kodeeditoren, hvor vi indsætter vores kode.

ClickFunnels er en smule forvirrende i det næste trin.

Det er vigtigt, at du *IKKE* vælger `Code`, når du holder musen over det nye element. Vælg i stedet `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Vælg SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Vælg SETTINGS" />
</div>

Nu kan vi i højre side klikke på `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Klik på Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Klik på Open Code Editor" />
</div>

Du vil se et stort felt åbne. Her kan vi indsætte vores kode. Kopiér følgende uddrag (brug kopiér-knappen øverst til højre):

[inline-code-attrs-start title = 'ClickFunnels Streaming Chat-kodesnippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // nogle udbydere ændrer kodesnippet til at være async
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

Dette kodesnit er til vores Streaming Chat-produkt, som passer godt til videoer. Hvis du i stedet ønsker Live Commenting-widgetens kodesnit, som passer bedst til almindelige sider eller blogindlæg, ligger det i slutningen af denne vejledning.

Når vi indsætter kodesnittet i vinduet, bør det se sådan ud:

<div class="screenshot white-bg">
    <div class="title">Indsæt kode</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Indsæt kode" />
</div>

Nu skal vi blot lukke boksen:

<div class="screenshot white-bg">
    <div class="title">Luk</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Luk" />
</div>

Nu kan du forhåndsvise dine ændringer! Flyt endelig widgetten rundt og se, hvor du bedst kan lide den.

<div class="screenshot white-bg">
    <div class="title">Forhåndsvis</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Forhåndsvis" />
</div>

Succes! Glem ikke at teste på mobil!

<div class="screenshot white-bg">
    <div class="title">Succes!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Succes!" />
</div>