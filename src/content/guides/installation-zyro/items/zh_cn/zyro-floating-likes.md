FastComments 还支持 Zyro 的 Page Reacts（也称为浮动喜欢按钮）小部件。

你可以在本页的右下角看到它的效果！

1. 首先，获取代码：

[inline-code-attrs-start title = 'Zyro 浮动喜欢按钮 代码'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. 然后，在 Zyro 中打开站点构建器。
3. 转到左下角的“网站设置”。
4. 选择“集成”。
5. 将新代码添加到 `Custom code` 字段的 *末尾*，并发布您的站点。
6. 在预览模式下你看不到该小部件，但在发布后的站点版本中它会出现。

---