Jetzt, da wir uns im Template-Editor befinden, müssen wir entscheiden, wo wir die Kommentare bzw. den Live-Chat anzeigen möchten.

In diesem Beispiel fügen wir ihn direkt unter dem Video hinzu. Fahren Sie mit der Maus über das Element, um das Widget ans Ende hinzuzufügen, und klicken Sie auf `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Element hinzufügen</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Element hinzufügen" />
</div>

Wählen Sie `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">CUSTOM JS/HTML auswählen</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="CUSTOM JS/HTML auswählen" />
</div>

Öffnen wir jetzt den Code-Editor, in den wir unseren Code einfügen werden.

ClickFunnels ist bei diesem nächsten Schritt etwas verwirrend.

Es ist wichtig, dass Sie *NICHT* `Code` auswählen, wenn Sie mit der Maus über das neue Element fahren. Wählen Sie stattdessen `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">SETTINGS auswählen</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="SETTINGS auswählen" />
</div>

Jetzt können wir auf der rechten Seite auf `Open Code Editor` klicken:

<div class="screenshot white-bg">
    <div class="title">Klicken Sie auf Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Klicken Sie auf Open Code Editor" />
</div>

Sie sehen ein großes Feld geöffnet. Hier können wir unseren Code einfügen. Kopieren Sie den folgenden Ausschnitt (verwenden Sie die Kopierschaltfläche oben rechts):

[inline-code-attrs-start title = 'ClickFunnels Streaming-Chat-Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // einige Anbieter ändern den Codeausschnitt, sodass er asynchron ist
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

Dieser Codeausschnitt ist für unser Produkt Streaming Chat, das gut zu Videos passt. Wenn Sie stattdessen den Codeausschnitt für das Live Commenting-Widget möchten, der am besten zu regulären Seiten oder Blogposts passt, finden Sie ihn am Ende dieses Tutorials.

Wenn wir den Codeausschnitt in das Fenster einfügen, sollte es so aussehen:

<div class="screenshot white-bg">
    <div class="title">Code einfügen</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Code einfügen" />
</div>

Jetzt müssen wir nur noch das Feld schließen:

<div class="screenshot white-bg">
    <div class="title">Schließen</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Schließen" />
</div>

Jetzt können Sie Ihre Änderungen in der Vorschau ansehen! Verschieben Sie das Widget nach Belieben und sehen Sie, wo es Ihnen am besten gefällt.

<div class="screenshot white-bg">
    <div class="title">Vorschau</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Vorschau" />
</div>

Erfolg! Vergessen Sie nicht, die Darstellung auf Mobilgeräten zu testen!

<div class="screenshot white-bg">
    <div class="title">Erfolg!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Erfolg!" />
</div>