---
FastComments 也支持 Hostinger 的 Page Reacts（即浮动点赞按钮）小部件。

你可以在本页的右下角看到它的运行效果！

### 注意！

以下说明适用于 Hostinger Site Builder。如果你使用的是 Hostinger 的 *WordPress*，只需复制下面的代码，并使用 [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/) 将其添加到你的 WordPress 站点，WPCode 是一个免费且易用的插件，用于向站点添加小的代码片段。

1. 首先，获取代码：

[inline-code-attrs-start title = 'Hostinger 浮动点赞代码'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. 然后，在 Hostinger 中打开站点构建器。
3. 在左下角进入 Website Settings。
4. 选择 Integrations。
5. 将新代码添加到 `Custom code` 字段的 *末尾*，并发布你的网站。
6. 在预览模式下你看不到该小部件，但在发布后的站点上它会出现。

---