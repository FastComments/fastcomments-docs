Nun können wir den folgenden Codeausschnitt kopieren. Verwenden Sie die Kopier-Schaltfläche, die oben rechts im Ausschnitt erscheint.

Es gibt einige Dinge, die Sie im Code konfigurieren können; siehe Zeilen 4 bis 7.

[inline-code-attrs-start title = 'Squarespace – Kommentare für alle Seiten (Code)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // Ihre Tenant-ID

        function tryLoad() {
            // Versuch, für verschiedene Layouts zu laden
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...dann fügen Sie ihn in den Codebereich ein und klicken Sie auf Speichern. Es sollte so aussehen, mit dem Code im `FOOTER`-Block:

<div class="screenshot white-bg">
    <div class="title">Einfügen und Speichern</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Einfügen und Speichern" />
</div>

Wenn Sie Probleme haben, stellen Sie sicher, dass weiter unten nicht `"tenantId": "demo"` steht. Es sollte Ihre Tenant-ID anzeigen, wenn Sie angemeldet sind. Falls nicht, wenden Sie sich an den Support.