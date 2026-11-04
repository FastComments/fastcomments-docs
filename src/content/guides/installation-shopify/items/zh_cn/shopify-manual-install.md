如果您无法安装 [Shopify App Store 应用](https://apps.shopify.com/fastcomments)，仍然可以通过编辑主题来添加 FastComments。当您想接入已拥有的 FastComments 租户，或者在不支持应用的 Shopify 店面上嵌入评论时，这种方式很有用。

对于大多数商店，基于应用的安装是受支持的路径。只有在应用不适用时才使用此方法。

### 第 1 步：禁用 Shopify 的原生评论

在您的 Shopify 管理后台，前往 **Blog posts > Manage blogs**，打开每个博客，并在右侧面板中将 **Comments are disabled** 设置为禁用。保存。

这可以防止 Shopify 的内置评论功能与 FastComments 同时显示。

### 第 2 步：打开博客主题模板

在您的 Shopify 管理后台：

1. 前往 **Online Store > Themes**。
2. 在当前主题下，点击 **Actions > Edit code**。
3. 在左侧的文件浏览器中，打开 **Sections** 并点击 `main-article.liquid`。

这是 Shopify 渲染单篇博客文章时使用的模板。

### 第 3 步：粘贴 FastComments 代码片段

滚动到 `main-article.liquid` 的大约第 100 行，即文章正文结束的 `</div>` 之后。粘贴以下代码片段：

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

将 `"demo"` 替换为您自己在 [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret) 中的 Tenant ID。点击 **Save**。

### 第 4 步：授权您的商店域名

在您已上线的商店中打开一篇博客文章。如果您看到的是授权错误而不是评论组件，说明 FastComments 需要知道您的商店被允许使用此租户。请参阅 [域名错误](/guide-installation-shopify.html#shopify-domain-errors)。

### 将 FastComments 添加到其他页面

同样的代码片段可用于任何 Liquid 模板，包括产品页面、自定义页面和主页。将其粘贴到您希望显示评论的位置，如果需要为每个页面使用稳定的标识符，请调整 `urlId`（例如，在产品模板上使用 `urlId: "{{ product.id }}"`）。