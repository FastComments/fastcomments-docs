---
Hugo の組み込み Disqus サポートと同じ方法で、すべての投稿にコメントを添付するには、テーマの single テンプレート (`layouts/_default/single.html`) に1行追加します:

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

そのパーシャルは `tenantId` が設定されている場合にのみレンダリングされます。個別のページでコメントを無効にするには、フロントマターで次のように設定します:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---