Ниже приведён фрагмент FastComments для Framer Live Comments.

[inline-code-attrs-start title = 'Фрагмент FastComments для Framer — комментарии'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // некоторые провайдеры делают фрагмент кода асинхронным
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

Or, alternatively, you can use the Streaming Chat widget. The Framer Streaming Chat FastComments snippet is:

[inline-code-attrs-start title = 'Фрагмент FastComments для Framer — чат в реальном времени'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // некоторые провайдеры делают фрагмент кода асинхронным
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

FastComments поддерживает редактор Framer, поэтому после вставки кода вы должны увидеть что-то похожее (возможно, вам потребуется нажать `Publish`):

<div class="screenshot white-bg">
    <div class="title">Предпросмотр виджета комментариев</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Предпросмотр виджета комментариев" />
</div>

Теперь при просмотре сайта вы должны увидеть область комментариев! В боковой панели Framer вы также можете установить виджет на всю ширину, если хотите.

Обратите внимание, что Framer ограничивает высоту виджетов и не поддерживает автоизменение размера, поэтому здесь мы выбрали виджет Live Chat
поскольку он имеет фиксированную высоту.