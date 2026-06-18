Да бисте прикључили коментаре на сваки пост онако како то ради уграђена Disqus подршка у Hugo, додајте један ред у single шаблон ваше теме (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Овај partial се рендерује само када је конфигурисан `tenantId`. Онемогућите коментаре на појединачној страници помоћу front matter-а:

```toml
+++
title = "A page with no comments"
comments = false
+++
```