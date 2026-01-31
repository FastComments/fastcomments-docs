现在让我们添加小部件代码。

复制下面的代码。请确保您已登录到 [fastcomments.com](https://fastcomments.com) 
并在未登录时重新加载此页面，以便代码会预填充您的帐户信息，否则它将显示演示代码。

现在复制代码：

[inline-code-attrs-start title = 'Zyro 评论代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

现在返回我们的网站构建器并点击 `Enter code`：

<div class="screenshot white-bg">
    <div class="title">输入代码</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="输入代码" />
</div>

### 注意！

请务必使用上面的代码，而不是其他文档中的代码片段，因为此代码片段是专为 Zyro 定制的。

现在您应该看到类似下面的内容，页面看起来是空白的。这是预期行为。将鼠标移动到小部件应该出现的区域：

<div class="screenshot white-bg">
    <div class="title">已添加代码小部件</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="已添加代码小部件" />
</div>

现在拖动小部件到所需大小，您会看到它出现：

<div class="screenshot white-bg">
    <div class="title">调整大小</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="调整大小" />
</div>

...现在预览并保存！