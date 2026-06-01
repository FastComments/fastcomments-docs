---
Pour attacher des commentaires à chaque article de la même manière que la prise en charge intégrée de Disqus dans Hugo, ajoutez une ligne au modèle single de votre thème (`layouts/_default/single.html`) :

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Le partial ne s'affiche que lorsque un `tenantId` est configuré. Désactivez les commentaires sur une page individuelle avec le front matter :

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---