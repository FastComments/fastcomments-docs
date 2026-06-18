For at vedhæfte kommentarer til hvert indlæg på samme måde som Hugos indbyggede Disqus-understøttelse gør, tilføj én linje til temaets single-skabelon (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Partialen gengives kun, når en `tenantId` er konfigureret. Deaktiver kommentarer på en individuel side med front-matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```