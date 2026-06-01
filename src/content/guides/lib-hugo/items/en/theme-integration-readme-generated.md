To attach comments to every post the way Hugo's built-in Disqus support does, add one line to your theme's single template (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

The partial renders only when a `tenantId` is configured. Disable comments on an individual page with front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```