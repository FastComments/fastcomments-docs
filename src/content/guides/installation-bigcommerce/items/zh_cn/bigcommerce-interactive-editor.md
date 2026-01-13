不建议通过 BigCommerce 的页面构建器添加 FastComments，因为这样必须将代码手动添加到每个需要的页面。

但是，如果确实想这样做，必须使用以下代码片段。由于 BigCommerce 的特性，其他教程中的代码片段将无法工作：

[inline-code-attrs-start title = 'BigCommerce 页面构建器片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        function attemptLoad() {
            if (loaded) {
                return;
            }
            if (!window.FastCommentsUI) {
                return;
            }
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo"
            });
            loaded = true;
        }
        attemptLoad();
        const interval = setInterval(function () {
            attemptLoad();
            if (loaded) {
                clearInterval(interval);
            }
        }, 300);
    })();
</script>
[inline-code-end]

---