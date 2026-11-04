小工具是一個 script 標籤和一個容器元素，因此可以直接放入你的 val 已經渲染的內容中。此範例使用 Hono JSX，這正是 Val Town 的 HTTP 範本所使用的技術。

[inline-code-attrs-start title = 'HTTP val 中的留言小工具'; type='javascript' inline-code-attrs-end]
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

## 在上線前先決定 urlId

`urlId` 決定留言會落在哪個討論串中。若不設定，它會預設為目前頁面 URL 經過清理後的版本，而這正是 Val Town 上會變動的部分：在你認領子網域之前，val 會有一個很長的 `*.web.val.run` 主機名稱、分支會有各自的 URL，而重新命名頁面則會改變路徑。每一種變化都會默默變成一個獨立且空白的討論串，症狀看起來就像「我的留言消失了」。

請將它設定為你可控的穩定值，例如文章的 slug 或資料庫 id，如上例所示。同時也傳入 `url`，這樣通知郵件與審核工具才能連回實際頁面。

## 在沒有 JavaScript 的情況下保留留言

FastComments 會在伺服器端渲染完整的討論串，val 可以將其放入 `<noscript>` 區塊中：

[inline-code-attrs-start title = '無 JavaScript 的備援方案'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

請對參數進行 URL 編碼。伺服器端版本支援匿名與登入留言、SSO 以及巢狀回覆。