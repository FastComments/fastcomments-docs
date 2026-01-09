Poniżej znajduje się fragment FastComments dla Framer Live Comments.

[inline-code-attrs-start title = 'Specyficzny fragment komentarzy FastComments dla Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // niektórzy dostawcy zmieniają fragment kodu na asynchroniczny
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

Lub, alternatywnie, możesz użyć widżetu czatu strumieniowego. Fragment FastComments dla Framer — Streaming Chat — wygląda tak:

[inline-code-attrs-start title = 'Specyficzny fragment czatu strumieniowego FastComments dla Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // niektórzy dostawcy zmieniają fragment kodu na asynchroniczny
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
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

FastComments obsługuje edytor Framer, więc po wklejeniu kodu powinieneś zobaczyć coś takiego (może być konieczne kliknięcie `Publish`):

<div class="screenshot white-bg">
    <div class="title">Podgląd widżetu komentarzy</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Podgląd widżetu komentarzy" />
</div>

Po odwiedzeniu swojej strony powinieneś zobaczyć obszar komentarzy! W pasku bocznym Framer możesz też ustawić widżet na pełną szerokość, jeśli chcesz.

Zwróć uwagę, że Framer ogranicza wysokość widżetów i nie obsługuje automatycznego dopasowywania rozmiaru, więc wybraliśmy tutaj widżet Live Chat, ponieważ ma stałą wysokość.