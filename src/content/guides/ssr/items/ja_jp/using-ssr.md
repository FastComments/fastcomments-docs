FastComments SSR を利用するには、クライアントが `https://fastcomments.com/ssr/comments` エンドポイントから HTML を取得します。

これにはいくつかの方法があります。

### WordPress の場合

WordPress プラグインでは、バージョン `3.10.2` 以降、JS が無効なユーザー向けのフォールバックとして、SSR がデフォルトで有効になっています。

### Web ページの場合

既存のアプリケーションでは、使用している言語が PHP であると仮定して、[次の例](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)のように SSR を追加できます。

[inline-code-attrs-start title = 'PHP を使用した SSR の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

ユーザーが JS を無効にしている場合にのみ SSR UI を表示することもできます:

[inline-code-attrs-start title = 'PHP を使用した SSR フォールバックの例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

SSO を使用した例については、[こちら](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)を参照してください。

### 事前レンダリングされたコンテンツの場合

私たちのブログはビルド時に生成されており、[Handlebars を使用した SSR の良い例](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51)を提供しています。

### 基本パラメーター

渡す必要がある基本パラメーターは以下の通りです:
- `tenantId` - 顧客としてあなたを識別します。
- `urlId` - コメントを読み込むページまたは記事を識別し、コメントの保存先を定義します。
- `url` - 通知や関連機能でコメントスレッドへのリンクに使用されます。

### カスタムスタイリング

コメントウィジェットの SSR 版は、JavaScript 版と同じ構造とレンダリングエンジンを使用しています。

そのため、JavaScript コメントウィジェットで機能するすべてのカスタムスタイリングは、SSR でも機能します。

### 注意事項

SSR では、レンダリングされたコンテナの高さを制御する JavaScript がありません。ブラウザでは、長いディスカッションの場合に垂直スクロールバーが表示されることがあります。

そのため、必要に応じて調整する必要があります。