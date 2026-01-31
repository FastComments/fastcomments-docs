现在我们可以复制以下代码片段。使用片段右上角出现的复制按钮。

有一些可以在代码中配置的内容，请参阅第 4 行到第 7 行。

[inline-code-attrs-start title = 'Squarespace 单页代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // 您的帐户 ID
    }];
</script>
[inline-code-end]

它看起来应该像这样：

<div class="screenshot white-bg">
    <div class="title">粘贴并保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="粘贴并保存" />
</div>

现在点击右上角的保存。

请注意，`Preview in Safe Mode` 选项将无法使用，但当您访问自己的网站时小部件会出现。

如果您遇到问题，请确保页面底部没有显示 `"tenantId": "demo"`。如果您已登录，它应该显示您的租户 ID。如果没有，请联系支持团队。