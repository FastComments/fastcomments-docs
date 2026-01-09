现在我们可以复制下面的代码片段。使用片段右上角出现的复制按钮。

有一些代码中的配置项，可查看第 4 到第 7 行。

[inline-code-attrs-start title = 'Squarespace 单页面代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 你的账户 id

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

应如下所示：

<div class="screenshot white-bg">
    <div class="title">粘贴并保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="粘贴并保存" />
</div>

现在在右上角点击保存。

请注意，`Preview in Safe Mode` 选项将无法使用，但当你访问你的网站时，小部件会出现。

如果你遇到问题，请确认底部附近没有显示 `"tenantId": "demo"`。如果你已登录，它应显示你的租户 id。如果没有，请联系支持。