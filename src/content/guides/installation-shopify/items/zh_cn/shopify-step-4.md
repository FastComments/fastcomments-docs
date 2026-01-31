---
接下来我们将向下滚动到第 `100` 行：

<div class="screenshot white-bg">
    <div class="title">滚动到第 100 行</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="滚动到第 100 行" />
</div>

现在复制以下代码片段，该片段是**专为 Shopify 设计的 - 不要使用其他教程中的代码片段**：

[inline-code-attrs-start title = 'Shopify FastComments 代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

现在我们要把光标放在 `line 101` - 紧接在 `</div>` 之后 - 然后粘贴。你应该会看到类似下面的内容：

<div class="screenshot white-bg">
    <div class="title">添加 FastComments 代码</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="添加 FastComments 代码" />
</div>

现在我们可以保存：

<div class="screenshot white-bg">
    <div class="title">保存</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="保存" />
</div>
---