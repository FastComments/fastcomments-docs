---
Да бисте додали коментаре на сваки пост на исти начин као што то ради уграђена подршка за Disqus у Hugo-у, додајте једну линију у single шаблон ваше теме (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Део шаблона (partial) ће се приказати само када је `tenantId` конфигурисан. Искључите коментаре на појединачној страници помоћу front matter-а:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---