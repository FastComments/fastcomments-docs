Om reacties aan elk bericht toe te voegen zoals de ingebouwde Disqus-ondersteuning van Hugo doet, voeg één regel toe aan de single-template van je thema (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

De partial wordt alleen weergegeven wanneer een `tenantId` is geconfigureerd. Schakel reacties uit op een individuele pagina via de front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```