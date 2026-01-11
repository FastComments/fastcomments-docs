FastComments SSR を使用するには、クライアントが `https://fastcomments.com/ssr/comments` エンドポイントから HTML を取得できます。

これはいくつかの方法で行うことができます。

### With WordPress

SSR は、WordPress プラグインのフォールバックとして JS が無効なユーザー向けにバージョン `3.10.2` 以降でデフォルトで有効になっています。

### In a Webpage

既存のアプリケーションでも、使用言語が PHP と仮定すると、[以下の例](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr) を使って SSR を追加できます:

[inline-code-attrs-start title = 'PHPベースのSSR例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

ユーザーが JS を無効にしている場合のみ SSR UI を表示することもできます:

[inline-code-attrs-start title = 'PHPベースのSSRフォールバック例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

SSO を使用した例については、[こちらを参照してください](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)。

### With Pre-Rendered Content

当社のブログはビルド時に生成されており、Handlebars を使った SSR の[良い例](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51) を示しています。

### The Basic Parameters

渡す必要がある基本的なパラメータは次のとおりです:
- `tenantId` - これはあなたを顧客として識別します。
- `urlId` - これはコメントを読み込むページや記事を識別し、保存先を定義します。
- `url` - これは通知や関連機能でコメントスレッドに戻るリンクとして使用されます。

### Custom Styling

コメントウィジェットの SSR バージョンは、JavaScript のものと同じ構造およびレンダリングエンジンを使用します。

したがって、JavaScript コメントウィジェットで動作するすべてのカスタムスタイリングは SSR でも動作します。 

### Notes

SSR では、レンダリングされたコンテナの高さを制御する JavaScript がありません。ブラウザでは、長い議論の場合に垂直スクロールバーが表示されることがあります。

そのため、必要に応じてこれを調整してください。