The Framer Live Comments FastComments снипет је испод.

[inline-code-attrs-start title = 'FastComments Framer — снипет за коментаре'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // неки провајдери мењају снипет кода да би био асинхрон
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

Или, као алтернатива, можете користити Streaming Chat видгет. FastComments снипет за Framer Streaming Chat је:

[inline-code-attrs-start title = 'FastComments Framer — снипет за стриминг ћаскање'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // неки провајдери мењају снипет кода да би био асинхрон
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

FastComments подржава Framer едитор, па бисте требали видјети нешто слично овоме након што налепите код (можда ћете морати кликнути на `Publish`):

<div class="screenshot white-bg">
    <div class="title">Преглед видџета за коментаре</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Преглед видџета за коментаре" />
</div>

Сада, када погледате ваш сајт, требало би да видите област за коментаре! У бочној траци у Framer-у такође можете подесити видџет да буде пуне ширине, ако желите.

Имајте у виду да Framer ограничава висину видџета и не подржава аутоматско ресајзовање, па смо овде изабрали Live Chat
видџет јер има фиксну висину.