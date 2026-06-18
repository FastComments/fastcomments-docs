要像 Hugo 内置的 Disqus 支持那样将评论附加到每篇文章，请在主题的 single 模板 (`layouts/_default/single.html`) 中添加一行：

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

仅当配置了 `tenantId` 时该 partial 才会渲染。要在单个页面上禁用评论，请在 front matter 中设置：

```toml
+++
title = "A page with no comments"
comments = false
+++
```