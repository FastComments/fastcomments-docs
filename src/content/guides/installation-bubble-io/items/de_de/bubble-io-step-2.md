Klicken Sie auf das HTML-Element, das Sie gerade hinzugefügt haben. Fügen Sie im daraufhin erscheinenden Eigenschaftseditor den folgenden Code in das HTML-Feld ein:

[inline-code-attrs-start title = 'Bubble.io Live-Kommentar-Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble neigt dazu, den Code-Snippet in asynchronen Code zu ändern
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">FastComments-Code einfügen</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="FastComments-Code in HTML-Element einfügen" />
</div>

Hinweis: Dieser Code enthält einen Wiederholungsmechanismus, um sicherzustellen, dass FastComments in der dynamischen Umgebung von Bubble korrekt geladen wird.
Andere Code-Snippets funktionieren nicht.

Denken Sie daran, `'demo'` nach der Anmeldung durch Ihre tatsächliche FastComments-Tenant-ID zu ersetzen. Wenn Sie bei FastComments.com angemeldet sind, sollte es bereits ersetzt worden sein.