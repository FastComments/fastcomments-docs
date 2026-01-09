Klik op het HTML-element dat u zojuist hebt toegevoegd. Plak in de eigenschappeneditor die verschijnt de volgende code in het HTML-veld:

[inline-code-attrs-start title = 'Bubble.io codefragment voor live commentaar'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble heeft de neiging het codefragment asynchroon te maken
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
    <div class="title">FastComments-code invoegen</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="FastComments-code invoegen in het HTML-element" />
</div>

Note: Deze code bevat een retry-mechanisme om ervoor te zorgen dat FastComments goed wordt geladen in de dynamische omgeving van Bubble. Andere codefragmenten zullen niet werken.

Vergeet niet `'demo'` te vervangen door uw daadwerkelijke FastComments tenant-ID nadat u zich heeft aangemeld. Als u bent ingelogd op FastComments.com, zou dit al vervangen moeten zijn.