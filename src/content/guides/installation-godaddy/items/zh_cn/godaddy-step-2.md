现在您已添加自定义 HTML 块，我们可以添加 FastComments 小部件代码。

**请使用下面针对 Godaddy 的代码，不要使用其他教程的代码。此代码为 Godaddy 专用。**

复制以下代码：

[inline-code-attrs-start title = 'Godaddy 评论代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

此代码段专为与 Godaddy 兼容而设计，并且只会显示在您的博客文章中——不会显示在主页上。

现在将代码粘贴到 `Custom Code` 区域（在 `Step One` 中提到）。

<div class="screenshot white-bg">
    <div class="title">复制并粘贴代码</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="复制并粘贴代码" />
</div>

在右上角点击完成：

<div class="screenshot white-bg">
    <div class="title">点击“完成”</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="点击“完成”" />
</div>

第二步就完成了！

---