Фрагмент FastComments для живих коментарів у Framer наведено нижче.

[inline-code-attrs-start title = 'Фрагмент коментарів FastComments для Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // деякі провайдери змінюють фрагмент коду на асинхронний
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

Або ж ви можете скористатися віджетом Streaming Chat. Фрагмент FastComments Streaming Chat для Framer виглядає так:

[inline-code-attrs-start title = 'Фрагмент Streaming Chat FastComments для Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // деякі провайдери змінюють фрагмент коду на асинхронний
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

FastComments підтримує редактор Framer, тому після вставки коду ви маєте побачити щось подібне (можливо, доведеться натиснути `Publish`):

<div class="screenshot white-bg">
    <div class="title">Попередній перегляд віджета коментарів</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Попередній перегляд віджета коментарів" />
</div>

Тепер, коли ви переглядатимете свій сайт, ви побачите область коментарів! У бічній панелі Framer за бажанням можна також налаштувати віджет на всю ширину.

Зверніть увагу, що Framer обмежує висоту віджетів і не підтримує автоматичне зміщення розміру, тому тут ми обрали віджет Live Chat, оскільки він має фіксовану висоту.