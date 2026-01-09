Kliknij element HTML, który właśnie dodałeś. W edytorze właściwości, który się pojawi, wklej poniższy kod do pola HTML:

[inline-code-attrs-start title = 'Fragment kodu komentowania na żywo dla Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble ma tendencję do zmieniania fragmentu kodu na async
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
    <div class="title">Wstaw kod FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Wstawianie kodu FastComments do elementu HTML" />
</div>

Uwaga: Ten kod zawiera mechanizm ponawiania prób, aby zapewnić prawidłowe załadowanie FastComments w dynamicznym środowisku Bubble.
Inne fragmenty kodu nie będą działać.

Pamiętaj, aby po rejestracji zastąpić `'demo'` swoim rzeczywistym tenant ID FastComments. Jeśli jesteś zalogowany na FastComments.com, powinno być to już zastąpione.