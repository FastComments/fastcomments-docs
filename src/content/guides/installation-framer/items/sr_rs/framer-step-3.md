FastComments исечак за Framer Live Comments је приказан испод.

[inline-code-attrs-start title = 'FastComments Framer-специфични исечак за коментаре'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Или, алтернативно, можете користити Streaming Chat виџет. FastComments исечак за Framer Streaming Chat је:

[inline-code-attrs-start title = 'FastComments Framer-специфични исечак за Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

FastComments подржава Framer едитор, тако да би требало да видите нешто овако када налепите код (можда ћете морати да кликнете `Publish`):

<div class="screenshot white-bg">
    <div class="title">Преглед виџета за коментаре</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Преглед виџета за коментаре" />
</div>

Сада када погледате свој сајт, требало би да видите област за коментаре! У бочној траци Framer-а можете, ако желите, поставити и виџет на пуну ширину.

Имајте у виду да Framer ограничава висину виџета и не подржава аутоматску промену величине, па смо овде изабрали Live Chat
виџет јер има фиксну висину.