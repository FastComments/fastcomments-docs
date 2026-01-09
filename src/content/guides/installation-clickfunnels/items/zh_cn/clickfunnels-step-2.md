现在我们在模板编辑器中，必须决定要在何处显示评论或实时聊天。

在这个示例中，我们将其直接添加到视频下方。将鼠标悬停在元素上以将小部件添加到其末尾，然后点击 `ADD ELEMENT`：

<div class="screenshot white-bg">
    <div class="title">添加元素</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="添加元素" />
</div>

选择 `CUSTOM JS/HTML`：

<div class="screenshot white-bg">
    <div class="title">选择 CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="选择 CUSTOM JS/HTML" />
</div>

现在我们打开代码编辑器，在那里我们将粘贴代码。

ClickFunnels 在下一步有点令人困惑。

重要的是，当您将鼠标悬停在新元素上时，切勿选择 `Code`。相反，请选择 `SETTINGS`：

<div class="screenshot white-bg">
    <div class="title">选择 SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="选择 SETTINGS" />
</div>

现在在右侧我们可以点击 `Open Code Editor`：

<div class="screenshot white-bg">
    <div class="title">点击 Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="点击 Open Code Editor" />
</div>

您会看到一个大方框打开。这就是我们可以粘贴代码的地方。复制以下代码片段（使用右上角的复制按钮）：

[inline-code-attrs-start title = 'ClickFunnels 流式聊天代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 某些提供商会将代码片段改为异步（async）
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

此代码片段用于我们的 Streaming Chat 产品，非常适合与视频一起使用。如果您想要用于常规页面或博客文章的 Live Commenting 小部件代码片段，请在本教程的末尾查看。

当我们将代码片段粘贴到窗口中时，它应如下所示：

<div class="screenshot white-bg">
    <div class="title">粘贴代码</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="粘贴代码" />
</div>

现在我们只需关闭该框：

<div class="screenshot white-bg">
    <div class="title">关闭</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="关闭" />
</div>

现在您可以预览您的更改！随意移动小部件，看看您最喜欢将它放在哪里。

<div class="screenshot white-bg">
    <div class="title">预览</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="预览" />
</div>

成功！别忘了测试移动端！

<div class="screenshot white-bg">
    <div class="title">成功！</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="成功！" />
</div>