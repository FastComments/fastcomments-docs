---
如果您无法安装 [Shopify 应用商店 应用](https://apps.shopify.com/fastcomments)，仍然可以通过编辑主题来添加 FastComments。当您想连接您已有的 FastComments 租户，或在应用不可用的 Shopify 商店前端进行嵌入时，此路径很有用。

基于应用的安装是大多数商店的受支持方式。只有在应用不合适时才采用此方法。

### 步骤 1：禁用 Shopify 的原生评论

在 Shopify 管理后台，转到 **博客文章 > 管理博客**，打开每个博客，并在右侧面板中将 **Comments are disabled** 设置为已禁用。保存。

这将阻止 Shopify 的内置评论与 FastComments 一起显示。

### 步骤 2：打开博客主题模板

在 Shopify 管理后台：

1. 转到 **在线商店 > 主题**。
2. 在当前主题下，点击 **操作 > 编辑代码**。
3. 在左侧的文件浏览器中，打开 **Sections** 并点击 `main-article.liquid`。

这是 Shopify 用于渲染单个博客文章的模板。

### 步骤 3：粘贴 FastComments 代码片段

滚动到 `main-article.liquid` 的大约第 100 行，就在文章主体的 `</div>` 关闭标签之后。粘贴以下片段：

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

Replace `"demo"` with your own Tenant ID from [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Click **Save**.

### 步骤 4：授权您的商店域名

在您的在线商店打开一篇博客文章。如果您看到的是授权错误而不是评论小部件，说明 FastComments 需要知道您的商店被允许使用此租户。参见 [域名错误](/guide-installation-shopify.html#shopify-domain-errors)。

### 将 FastComments 添加到其他页面

相同的片段适用于任何 Liquid 模板，包括产品页面、自定义页面以及主页。将其粘贴到希望显示评论的位置，并在每页需要稳定标识符时调整 `urlId`（例如，在产品模板上使用 `urlId: "{{ product.id }}"`）。 

---