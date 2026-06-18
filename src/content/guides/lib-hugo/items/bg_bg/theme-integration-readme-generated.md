За да прикачите коментари към всяка публикация по същия начин, както вградената в Hugo поддръжка за Disqus, добавете един ред в single шаблона на темата си (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Частичният шаблон се рендерира само когато е конфигуриран `tenantId`. Забранете коментарите за отделна страница чрез front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```