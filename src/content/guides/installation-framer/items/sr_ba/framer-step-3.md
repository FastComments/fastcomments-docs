The Framer Live Comments FastComments исјечак је испод.

[inline-code-attrs-start title = 'FastComments Framer-специфични исјечак за коментаре'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // неки провајдери мењају исјечак кода да буде асинхроно
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

[inline-code-attrs-start title = 'FastComments Framer-специфични исјечак за Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // неки провајдери мењају исјечак кода да буде асинхроно
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

FastComments подржава Framer уређивач, па бисте требали видјети нешто слично овоме након што залепите код (можда ћете морати да кликнете `Publish`):

<div class="screenshot white-bg">
    <div class="title">Преглед видгета за коментаре</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Преглед видгета за коментаре" />
</div>

Сада када прегледате ваш сајт требало би да видите област за коментаре! У бочној траци Framer-а такође можете поставити видгет да буде пуне ширине, ако желите.

Имајте на уму да Framer ограничава висину видгета и не подржава аутоматско прилагођавање величине, па смо овдје одабрали Live Chat видгет јер има фиксну висину.