---
Aby dołączyć komentarze do każdego wpisu tak jak robi to wbudowane wsparcie Disqus w Hugo, dodaj jedną linię do szablonu single swojego motywu (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Ten partial jest renderowany tylko wtedy, gdy skonfigurowano `tenantId`. Wyłącz komentarze na pojedynczej stronie za pomocą front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---