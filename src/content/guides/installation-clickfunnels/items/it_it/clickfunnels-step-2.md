Ora che siamo nell'editor del template, dobbiamo decidere dove vogliamo visualizzare i commenti, o la chat in diretta.

In questo esempio lo aggiungeremo direttamente sotto il video. Passa il mouse sull'elemento per aggiungere il widget alla fine, e clicca `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Aggiungi elemento</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Aggiungi elemento" />
</div>

Select `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Seleziona CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Seleziona CUSTOM JS/HTML" />
</div>

Ora apriamo l'editor del codice dove incolleremo il nostro codice.

ClickFunnels è un po' confuso in questo passaggio successivo.

È importante che tu *NON* selezioni `Code` quando passi il mouse sul nuovo elemento. Seleziona invece `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Seleziona IMPOSTAZIONI</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Seleziona IMPOSTAZIONI" />
</div>

Ora, sul lato destro possiamo cliccare `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Clicca Apri editor codice</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Clicca Apri editor codice" />
</div>

Vedrai aprirsi un grande riquadro. Qui possiamo incollare il nostro codice. Copia lo snippet seguente (usa il pulsante copia in alto a destra):

[inline-code-attrs-start title = 'Snippet di codice Chat in streaming per ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // alcuni provider modificano lo snippet di codice rendendolo asincrono
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

Questo snippet di codice è per il nostro prodotto Streaming Chat, che si abbina bene con i video. Se invece vuoi lo snippet di codice del widget Live Commenting, che è più adatto a pagine normali o post del blog, si trova alla fine di questo tutorial.

Quando incolliamo lo snippet di codice nella finestra, dovrebbe apparire così:

<div class="screenshot white-bg">
    <div class="title">Incolla codice</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Incolla codice" />
</div>

Ora dobbiamo solo chiudere la finestra:

<div class="screenshot white-bg">
    <div class="title">Chiudi</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Chiudi" />
</div>

Ora puoi visualizzare l'anteprima delle modifiche! Sentiti libero di spostare il widget e vedere dove preferisci posizionarlo.

<div class="screenshot white-bg">
    <div class="title">Anteprima</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Anteprima" />
</div>

Successo! Non dimenticare di testare la versione mobile!

<div class="screenshot white-bg">
    <div class="title">Successo!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Successo!" />
</div>