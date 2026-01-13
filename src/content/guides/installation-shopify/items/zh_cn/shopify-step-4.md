接下来我们将向下滚动到第 `100` 行：

<div class="screenshot white-bg">
    <div class="title">滚动到第 100 行</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="滚动到第 100 行" />
</div>

现在复制下面的代码片段，该片段专为 Shopify 设计 **- 请勿使用其他教程中的代码片段**：

[inline-code-attrs-start title = 'Shopify FastComments 代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

现在我们要把光标放在 `line 101` 上 - 就在 `</div>` 之后 - 然后粘贴。你应该会看到类似这样的内容：

<div class="screenshot white-bg">
    <div class="title">添加 FastComments 代码</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="添加 FastComments 代码" />
</div>

现在我们可以保存：

<div class="screenshot white-bg">
    <div class="title">保存</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="保存" />
</div>