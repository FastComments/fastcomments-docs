现在我们可以复制以下代码片段。使用出现在代码片段右上角的复制按钮。

代码中有一些可配置项，参见第 4 到第 7 行。

[inline-code-attrs-start title = 'Squarespace 所有页面评论代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 你的帐户 ID

        function tryLoad() {
            // 尝试为不同布局加载
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...然后将代码粘贴到代码区域并点击保存。它应如下所示，代码位于 `FOOTER` 块：

<div class="screenshot white-bg">
    <div class="title">粘贴并保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="粘贴并保存" />
</div>

如果遇到问题，请确保底部附近没有显示 `"tenantId": "demo"`。如果你已登录，应该显示你的 tenant id。如果没有，请联系支持。