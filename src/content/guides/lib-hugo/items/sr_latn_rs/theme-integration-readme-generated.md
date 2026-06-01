---
Da biste prikačili komentare na svaki post na isti način na koji to radi Hugo-ova ugrađena podrška za Disqus, dodajte jednu liniju u single template vaše teme (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Partial se prikazuje samo kada je `tenantId` konfigurisan. Isključite komentare na pojedinačnoj stranici pomoću front matter-a:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---