The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = '在 HTTP val 中的評論小工具'; type='javascript' inline-code-attrs-end]
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

## 在上線前選擇 urlId

`urlId` decides which thread a comment lands in. Leave it unset and it defaults to a cleaned version of the current page URL, which is exactly the thing that changes on Val Town: a val has a long `*.web.val.run` hostname until you claim a subdomain, branches get their own URLs, and renaming a page changes the path. Each variation silently becomes a separate, empty thread, and the symptom reads as "my comments disappeared".

Set it to something stable that you control, like the post slug or a database id, as above. Pass `url` too, so notification emails and the moderation tools can link back to the real page.

## 在沒有 JavaScript 時保留評論

FastComments renders a full thread server-side, which a val can drop into a `<noscript>` block:

[inline-code-attrs-start title = '無 JavaScript 後備方案'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL-encode the parameters. The server-side version supports anonymous and logged-in commenting, SSO, and nested replies.

---