FastComments snippet-ът за Framer Live Comments е по-долу.

[inline-code-attrs-start title = 'FastComments Framer-специфичен snippet за коментари'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // някои доставчици правят кодовия snippet асинхронен
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

Или, като алтернатива, можете да използвате уиджета Streaming Chat. FastComments snippet-ът за Framer Streaming Chat е:

[inline-code-attrs-start title = 'FastComments Framer-специфичен snippet за Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // някои доставчици правят кодовия snippet асинхронен
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

FastComments поддържа редактора Framer, така че след като поставите кода, трябва да видите нещо подобно (може да се наложи да кликнете `Publish`):

<div class="screenshot white-bg">
    <div class="title">Преглед на уиджета за коментари</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Преглед на уиджета за коментари" />
</div>

Сега, когато видите сайта си, трябва да видите зоната за коментари! В страничната лента на Framer можете, ако желаете, да зададете и уиджета на пълна ширина.

Имайте предвид, че Framer ограничава височината на уиджетите и не поддържа автоматично преоразмеряване, затова тук избрахме уиджета Live Chat, тъй като той е с фиксирана височина.