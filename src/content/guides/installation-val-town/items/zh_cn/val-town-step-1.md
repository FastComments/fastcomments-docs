该组件是一个 script 标签和一个容器元素，因此可以嵌入到你现有的 val 渲染内容中。此示例使用 Hono JSX，这也是 Val Town 的 HTTP 模板所使用的技术。

[inline-code-attrs-start title = 'HTTP val 中的评论组件'; type='javascript' inline-code-attrs-end]
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

## 在上线之前先确定 urlId

`urlId` 决定评论归属于哪个讨论串。如果不设置，它默认使用当前页面 URL 的清理版本，而这恰恰是 Val Town 上会变化的东西：在你认领子域名之前，val 会一直使用较长的 `*.web.val.run` 主机名，分支会有各自的 URL，重命名页面会改变路径。每一种变化都会悄悄变成一个独立的、空的讨论串，其症状表现为“我的评论消失了”。

请像上面那样，将其设置为你可控的稳定值，例如文章的 slug 或数据库 id。同时也要传入 `url`，这样通知邮件和审核工具才能链接回真实页面。

## 在无 JavaScript 的情况下保留评论

FastComments 会在服务端渲染完整的讨论串，val 可以将其放入 `<noscript>` 块中：

[inline-code-attrs-start title = '无 JavaScript 的后备方案'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

请对参数进行 URL 编码。服务端版本支持匿名和登录评论、SSO 以及嵌套回复。