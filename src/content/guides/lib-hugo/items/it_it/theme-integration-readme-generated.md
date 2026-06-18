---
Per aggiungere commenti a ogni post, come fa il supporto Disqus integrato di Hugo, aggiungi una riga al template single del tuo tema (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

La partial viene visualizzata solo quando è configurato un `tenantId`. Disabilita i commenti su una singola pagina tramite il front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---