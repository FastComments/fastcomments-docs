要像 Hugo 內建的 Disqus 支援那樣為每篇文章附加評論，請在主題的 single 模板 (`layouts/_default/single.html`) 中加入一行：

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

該 partial 只有在設定了 `tenantId` 時才會呈現。可以在單一頁面的 front matter 中停用評論：

```toml
+++
title = "A page with no comments"
comments = false
+++
```