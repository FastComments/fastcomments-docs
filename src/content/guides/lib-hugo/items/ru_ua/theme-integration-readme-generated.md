---
Чтобы прикрепить комментарии к каждому посту так же, как это делает встроенная поддержка Disqus в Hugo, добавьте одну строку в single-шаблон вашей темы (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Этот partial отображается только тогда, когда настроен `tenantId`. Отключите комментарии на отдельной странице с помощью front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---