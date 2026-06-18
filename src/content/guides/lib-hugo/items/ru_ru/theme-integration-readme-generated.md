Чтобы прикрепить комментарии к каждому посту так же, как это делает встроенная поддержка Disqus в Hugo, добавьте одну строку в шаблон single вашей темы (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Частичный шаблон рендерится только тогда, когда настроен `tenantId`. Отключить комментарии на отдельной странице можно через front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```