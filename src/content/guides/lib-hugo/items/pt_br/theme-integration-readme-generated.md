---
Para anexar comentários a cada post da mesma forma que o suporte integrado ao Disqus do Hugo, adicione uma linha ao template single do seu tema (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

O partial é renderizado somente quando um `tenantId` está configurado. Desative comentários em uma página individual com front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---