Jetzt können wir den folgenden Codeausschnitt kopieren. Verwenden Sie die Kopier-Schaltfläche, die oben rechts im Ausschnitt erscheint.

Es gibt einige Dinge, die Sie im Code konfigurieren können — siehe Zeilen 4 bis 7.

[inline-code-attrs-start title = 'Squarespace Einzelseiten-Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // Ihre Konto-ID

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Es sollte so aussehen:

<div class="screenshot white-bg">
    <div class="title">Einfügen und Speichern</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Einfügen und Speichern" />
</div>

Klicken Sie jetzt oben rechts auf Speichern.

Beachten Sie, dass die Option `Preview in Safe Mode` nicht funktioniert, aber das Widget erscheint, wenn Sie Ihre Website besuchen.

Wenn Sie Probleme haben, stellen Sie sicher, dass unten nicht `"tenantId": "demo"` steht. Es sollte Ihre tenant id anzeigen, wenn Sie eingeloggt sind. Wenn nicht, wenden Sie sich an den Support.