---
Щоб додати коментарі до кожного запису так само, як це робить вбудована підтримка Disqus у Hugo, додайте один рядок до шаблону single вашої теми (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Частковий шаблон рендериться лише коли налаштований `tenantId`. Вимкніть коментарі на окремій сторінці за допомогою front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---