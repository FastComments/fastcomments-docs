Dieses Beispiel verwendet benutzerdefinierten Code, um mit Wix kompatibel zu sein. **Sie können die FastComments-Codeausschnitte aus anderen Tutorials nicht verwenden.**

Öffnen Sie das Formular, um unseren benutzerdefinierten HTML-Dialog hinzuzufügen, indem Sie auf `Enter Code` klicken und `HTML` auswählen:

<div class="screenshot white-bg">
    <div class="title">Schritt 3: HTML-Dialog öffnen</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Schritt 3: HTML-Dialog öffnen" />
</div>

Kopieren Sie den folgenden HTML-Snippet und fügen Sie ihn in den Dialog ein, und klicken Sie auf Update:

[inline-code-attrs-start title = 'Wix-Kommentare Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Schritt 3: Einfügen und Speichern</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Schritt 3: Einfügen und Speichern" />
</div>

Sie sollten nun eine sehr kleine Version des Kommentar-Widgets sehen:

<div class="screenshot white-bg">
    <div class="title">Schritt 3: Das Ergebnis</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Schritt 3: Das Ergebnis" />
</div>

Als Nächstes werden wir die Position und Größe anpassen, damit es auf unsere Seite passt.