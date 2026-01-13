要使用 FastComments SSR，客户端可以从 `https://fastcomments.com/ssr/comments` 端点获取 HTML。

这可以通过多种方式完成。

### 在 WordPress 中

自 WordPress 插件版本 `3.10.2` 起，SSR 在没有启用 JS 的用户中作为回退已默认启用。

### 在网页中

对于已有的应用，假设使用的语言是 PHP，可以通过 [以下示例](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr) 添加 SSR：

[inline-code-attrs-start title = '基于 PHP 的 SSR 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

我们也可以仅在用户禁用 JS 时显示 SSR 界面：

[inline-code-attrs-start title = '基于 PHP 的 SSR 回退 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

有关使用 SSO 的示例，请 [参见此处](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)。

### 使用预渲染内容

我们的博客在构建时生成，并提供一个 [使用 Handlebars 的 SSR 的良好示例](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51)。

### 基本参数

您需要传入的基本参数有：
- `tenantId` - 该参数用于识别您作为客户。
- `urlId` - 该参数用于识别要加载评论的页面或文章，并定义评论保存的位置。
- `url` - 该参数用于通知和相关功能，以链接回评论线程。

### 自定义样式

SSR 版本的评论小部件使用与 JavaScript 版本相同的结构和渲染引擎。

因此，所有适用于 JavaScript 评论小部件的自定义样式同样适用于 SSR。

### 注意事项

在 SSR 中，没有 JavaScript 来控制渲染容器的高度。在浏览器中，长讨论可能会显示垂直滚动条。

因此，您必须根据需要进行调整。