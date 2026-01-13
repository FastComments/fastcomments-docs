Nun können wir den folgenden Codeausschnitt kopieren (verwenden Sie die Kopier-Schaltfläche in der oberen rechten Ecke des Ausschnitts):

[inline-code-attrs-start title = 'Squarespace Blog-Kommentare Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // Ihre Konto-ID

        function tryLoad() {
            // Versucht, für verschiedene Layouts zu laden
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

...dann fügen Sie ihn in den Codebereich ein und klicken auf Speichern:

<div class="screenshot white-bg">
    <div class="title">Einfügen und Speichern</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Einfügen und Speichern" />
</div>

---