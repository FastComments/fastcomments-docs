可以使用以下示例语法搜索评论：

- 模糊词搜索： `cats love`
- 精确短语匹配： `I love cats.`
- 精确完整评论匹配： `exact="I love cats."`
  - 只匹配整个评论文本完全等于该值的评论（区分大小写），而不是仅包含该文本的评论。
- 按页面标题： `page:"Page Title"`
  - 支持自动完成。
- 按页面 URL： `page:"https://example.com/some-page"`
  - 支持自动完成。
- 按站点/域名： `site:mysite.com` or `domain:othersite.com`
- 按用户： `user:"Bob"`
  - 支持自动完成。

您可以通过分享审核页面的页面 URL，将搜索结果与其他版主或管理员共享。搜索字段
的值将在您点击 "Go" 后包含在浏览器的 URL 中。

---