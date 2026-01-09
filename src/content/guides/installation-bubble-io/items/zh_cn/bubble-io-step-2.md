单击你刚添加的 HTML 元素。在出现的属性编辑器中，将以下代码粘贴到 HTML 字段：

[inline-code-attrs-start title = 'Bubble.io 实时评论 代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble 倾向于将代码片段更改为 async
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

<div class="screenshot white-bg">
    <div class="title">插入 FastComments 代码</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="将 FastComments 代码插入到 HTML 元素中" />
</div>

注意：此代码包含重试机制，以确保 FastComments 在 Bubble 的动态环境中正确加载。
其他代码片段将无法工作。

请记得在注册后将 'demo' 替换为您实际的 FastComments 租户 ID。如果您已登录 FastComments.com，它应该已经被替换。