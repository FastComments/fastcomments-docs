Um Kommentare an jeden Beitrag anzuhängen, so wie es die eingebaute Disqus-Unterstützung von Hugo macht, fügen Sie Ihrem Theme im Single-Template (`layouts/_default/single.html`) eine Zeile hinzu:

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Das Partial wird nur gerendert, wenn eine `tenantId` konfiguriert ist. Deaktivieren Sie Kommentare auf einer einzelnen Seite mit der Front Matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```