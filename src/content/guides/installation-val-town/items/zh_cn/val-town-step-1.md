The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = '在 HTTP val 中的评论小部件'; type='javascript' inline-code-attrs-end]
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

## 在发布前选择 urlId

`urlId` 决定评论落入哪个线程。如果不设置，它会默认使用当前页面 URL 的清理版本，这正是 Val Town 上会变化的东西：一个 val 在你声明子域之前会有一个长的 `*.web.val.run` 主机名，分支会拥有各自的 URL，重命名页面会改变路径。每一种变化都会悄悄地生成一个独立的空线程，表现为“我的评论消失了”。

将其设置为你可控制的稳定值，例如上面的文章 slug 或数据库 ID。同时也传递 `url`，以便通知邮件和审核工具能够链接回真实页面。

## 在没有 JavaScript 的情况下保留评论

FastComments 在服务器端渲染完整的线程，val 可以将其放入 `<noscript>` 块中：

[inline-code-attrs-start title = '无 JavaScript 回退'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

对参数进行 URL 编码。服务器端版本支持匿名和已登录的评论、单点登录（SSO）以及嵌套回复。