要使用 FastComments SSR，client 可以從 `https://fastcomments.com/ssr/comments` 端點擷取 HTML。

這可以用多種方式完成。

### 在 WordPress 中

自從 WordPress 外掛版本 `3.10.2` 起，當使用者停用 JS 時，SSR 預設會作為回退機制啟用。

### 在網頁中

在已存在的應用程式中，假設使用的語言是 PHP，SSR 可以透過 [以下範例](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr) 加入：

[inline-code-attrs-start title = 'PHP 範例：伺服端渲染 (SSR)'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

我們也可以僅在使用者停用 JS 時顯示 SSR UI：

[inline-code-attrs-start title = 'PHP 範例：SSR 回退'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

有關使用 SSO 的範例，請參見 [此處](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)。

### 使用預先渲染的內容

我們的部落格在建置時生成，並提供了一個 [使用 Handlebars 的良好 SSR 範例](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51)。

### 基本參數

您需要傳遞的基本參數為：
- `tenantId` - 這識別您作為客戶。
- `urlId` - 這識別要載入評論的頁面或文章，並決定評論儲存的位置。
- `url` - 用於通知及相關功能，以連回該評論串。

### 自訂樣式

SSR 版本的評論小工具使用與 JavaScript 版本相同的結構與渲染引擎。

因此，所有適用於 JavaScript 評論小工具的自訂樣式，也適用於 SSR。

### 注意事項

在 SSR 中，沒有 JavaScript 來控制已渲染容器的高度。在瀏覽器中，長篇討論可能會出現垂直捲軸。

因此，您必須依需要調整它。