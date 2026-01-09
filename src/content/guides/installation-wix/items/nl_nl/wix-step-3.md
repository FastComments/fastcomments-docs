Dit voorbeeld gebruikt aangepaste code om compatibel te zijn met Wix. **Je kunt de FastComments-codefragmenten uit andere handleidingen niet gebruiken.**

Open het formulier om onze aangepaste HTML-dialoog toe te voegen door op `Enter Code` te klikken en `HTML` te selecteren:

<div class="screenshot white-bg">
    <div class="title">Stap 3: HTML-dialoog openen</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Stap 3: HTML-dialoog openen" />
</div>

Kopieer het volgende HTML-fragment en plak het in de dialoog, en klik op Update:

[inline-code-attrs-start title = 'Wix Reacties Codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Stap 3: Plakken en opslaan</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Stap 3: Plakken en opslaan" />
</div>

Je zou nu een zeer kleine versie van de reacties-widget moeten zien:

<div class="screenshot white-bg">
    <div class="title">Stap 3: Het resultaat</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Stap 3: Het resultaat" />
</div>

Vervolgens positioneren en schalen we dit zodat het op onze pagina past.