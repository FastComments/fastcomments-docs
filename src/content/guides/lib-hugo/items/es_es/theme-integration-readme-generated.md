---
Para adjuntar comentarios a cada entrada de la misma manera que lo hace la compatibilidad integrada de Disqus de Hugo, añade una línea al template single de tu tema (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

El partial se renderiza solo cuando se configura un `tenantId`. Deshabilita los comentarios en una página individual usando front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---