Nu we in de template-editor zitten, moeten we beslissen waar we de reacties, of live chat, willen weergeven.

In dit voorbeeld voegen we het direct onder de video toe. Beweeg de muis over het element waaraan we de widget aan het einde willen toevoegen en klik op `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Element toevoegen</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Element toevoegen" />
</div>

Selecteer `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Selecteer CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Selecteer CUSTOM JS/HTML" />
</div>

Nu openen we de code-editor waar we onze code plakken.

ClickFunnels is een beetje verwarrend bij deze volgende stap.

Het is belangrijk dat je *NIET* `Code` selecteert wanneer je met de muis over het nieuwe element beweegt. Selecteer in plaats daarvan `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Selecteer SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Selecteer SETTINGS" />
</div>

Nu kunnen we aan de rechterkant op `Open Code Editor` klikken:

<div class="screenshot white-bg">
    <div class="title">Klik op Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Klik op Open Code Editor" />
</div>

Je ziet een groot vierkant verschijnen. Dit is waar we onze code kunnen plakken. Kopieer het volgende fragment (gebruik de kopieerknop rechtsboven):

[inline-code-attrs-start title = 'ClickFunnels Streaming Chat Codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // sommige providers wijzigen het codefragment zodat het asynchroon is
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

Dit codefragment is voor ons Streaming Chat-product, dat goed past bij video's. Als je in plaats daarvan het codefragment van de Live Commenting-widget wilt, dat het beste werkt op gewone pagina's of blogposts, vind je het aan het einde van deze tutorial.

Wanneer we het codefragment in het venster plakken, ziet het er als volgt uit:

<div class="screenshot white-bg">
    <div class="title">Code plakken</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Code plakken" />
</div>

Nu hoeven we alleen nog het venster te sluiten:

<div class="screenshot white-bg">
    <div class="title">Sluiten</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Sluiten" />
</div>

Nu kun je je wijzigingen bekijken! Beweeg de widget gerust rond en kijk waar je hem het liefst hebt.

<div class="screenshot white-bg">
    <div class="title">Voorbeeld</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Voorbeeld" />
</div>

Gelukt! Vergeet niet om het op mobiel te testen!

<div class="screenshot white-bg">
    <div class="title">Gelukt!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Gelukt!" />
</div>