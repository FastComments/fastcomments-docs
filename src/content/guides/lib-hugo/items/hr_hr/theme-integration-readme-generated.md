Da biste dodali komentare svakom objavi na način na koji to radi ugrađena Hugo podrška za Disqus, dodajte jedan redak u single predložak vaše teme (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Partial se prikazuje samo kada je konfiguriran `tenantId`. Onemogućite komentare na pojedinačnoj stranici pomoću front mattera:

```toml
+++
title = "A page with no comments"
comments = false
+++
```