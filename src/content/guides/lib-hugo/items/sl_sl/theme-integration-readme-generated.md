Če želite priložiti komentarje k vsakemu prispevku na enak način, kot to stori Hugojeva vgrajena podpora za Disqus, dodajte eno vrstico v predlogo single vaše teme (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Delni predlogo (partial) se prikaže le, ko je konfiguriran `tenantId`. Komentarje onemogočite na posamezni strani z uporabo front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```