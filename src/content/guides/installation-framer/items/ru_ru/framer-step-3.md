Сниппет FastComments для Framer Live Comments приведён ниже.

[inline-code-attrs-start title = 'Сниппет комментариев FastComments специально для Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
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

Или, в качестве альтернативы, вы можете использовать виджет Streaming Chat. Сниппет FastComments для Framer Streaming Chat выглядит так:

[inline-code-attrs-start title = 'Сниппет Streaming Chat FastComments специально для Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // some providers change the code snippet to be async
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

FastComments поддерживает редактор Framer, поэтому после вставки кода вы должны увидеть примерно следующее (возможно, потребуется нажать `Publish`):

<div class="screenshot white-bg">
    <div class="title">Предварительный просмотр виджета комментариев</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Предварительный просмотр виджета комментариев" />
</div>

Теперь при просмотре вашего сайта вы должны увидеть область комментариев! В боковой панели Framer при желании можно также задать виджету полную ширину.

Обратите внимание, что Framer ограничивает высоту виджетов и не поддерживает автоматическое изменение размера, поэтому здесь мы выбрали виджет Live Chat, так как он имеет фиксированную высоту.