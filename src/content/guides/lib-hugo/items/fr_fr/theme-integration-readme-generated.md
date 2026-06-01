Pour ajouter des commentaires à chaque article, comme le fait le support Disqus intégré de Hugo, ajoutez une ligne au modèle single de votre thème (`layouts/_default/single.html`) :

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Le partial ne s'affiche que lorsqu'un `tenantId` est configuré. Désactivez les commentaires sur une page individuelle avec le front matter :

```toml
+++
title = "A page with no comments"
comments = false
+++
```