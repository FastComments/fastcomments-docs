ウィジェットは script タグとコンテナ要素で構成されているため、val がすでにレンダリングしている場所にそのまま埋め込まれます。この例では Hono JSX を使用していますが、これは Val Town の HTTP テンプレートで使用されているものです。

[inline-code-attrs-start title = 'HTTP val のコメントウィジェット'; type='javascript' inline-code-attrs-end]
[inline-code-start]
/** @jsxImportSource npm:hono@4/jsx */
import { Hono } from "npm:hono@4";

const app = new Hono();

app.get("/:slug", (c) => {
  const slug = c.req.param("slug");
  const url = new URL(c.req.path, c.req.url).toString();

  const config = JSON.stringify({
    tenantId: "demo",
    urlId: slug,
    url,
  });

  return c.html(
    <html>
      <body>
        <h1>{slug}</h1>
        <div id="fastcomments-widget"></div>
        <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
        <script
          dangerouslySetInnerHTML={{
            __html:
              `window.FastCommentsUI(document.getElementById("fastcomments-widget"), ${config});`,
          }}
        />
      </body>
    </html>,
  );
});

export default app.fetch;
[inline-code-end]

## 配信前に urlId を選択

`urlId` はコメントが属するスレッドを決定します。設定しない場合、現在のページ URL のクリーンアップされたバージョンがデフォルトとなります。これは Val Town で変化する要素そのものです。val はサブドメインを取得するまで長い `*.web.val.run` ホスト名を持ち、ブランチはそれぞれ独自の URL を持ち、ページの名前を変更するとパスが変わります。各バリエーションは静かに別々の空スレッドとなり、結果として「コメントが消えた」と感じます。

上記のように、投稿スラッグやデータベース ID など、あなたが管理できる安定した値に設定してください。また `url` も渡すことで、通知メールやモデレーションツールが実際のページへリンクできるようになります。

## JavaScript なしでコメントを保持する

[inline-code-attrs-start title = 'No-JavaScript フォールバック'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

パラメータは URL エンコードしてください。サーバーサイド版は匿名コメントとログインコメント、SSO、そして入れ子返信をサポートしています。