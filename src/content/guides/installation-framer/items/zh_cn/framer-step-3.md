Framer 实时评论 (Live Comments) 的 FastComments 代码片段如下。

[inline-code-attrs-start title = 'FastComments Framer 专用评论片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 某些提供商会将代码片段改为异步
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

或者，你也可以使用流式聊天（Streaming Chat）小部件。Framer 流式聊天的 FastComments 代码片段如下：

[inline-code-attrs-start title = 'FastComments Framer 专用实时聊天片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // 某些提供商会将代码片段改为异步
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

FastComments 支持 Framer 编辑器，因此在粘贴代码后你应该看到类似的效果（你可能需要点击 `Publish`）：

<div class="screenshot white-bg">
    <div class="title">评论小部件预览</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="评论小部件预览" />
</div>

现在当你查看你的网站时应该能看到评论区域！在 Framer 的侧边栏中，你也可以根据需要将小部件设置为全宽。

请注意，Framer 限制小部件的高度并且不支持自动调整大小，因此我们在此选择了固定高度的 Live Chat
widget。