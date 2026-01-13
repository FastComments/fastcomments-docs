Нижче наведено фрагмент FastComments для Framer Live Comments.

[inline-code-attrs-start title = 'Фрагмент FastComments для Framer (коментарі)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // деякі провайдери змінюють фрагмент коду, роблячи його асинхронним
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

[inline-code-attrs-start title = 'Фрагмент FastComments для Framer (потоковий чат)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // деякі провайдери змінюють фрагмент коду, роблячи його асинхронним
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

FastComments підтримує редактор Framer, тож ви повинні побачити щось подібне після вставлення коду (можливо, доведеться натиснути `Publish`):

<div class="screenshot white-bg">
    <div class="title">Попередній перегляд віджета коментарів</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Попередній перегляд віджета коментарів" />
</div>

Тепер, коли ви переглядаєте свій сайт, ви повинні побачити область коментарів! У бічній панелі Framer ви також можете встановити віджет на повну ширину, якщо бажаєте.

Зауважте, що Framer обмежує висоту віджетів і не підтримує автоматичне змінення розміру, тому тут ми обрали віджет потокового чату, оскільки він має фіксовану висоту.