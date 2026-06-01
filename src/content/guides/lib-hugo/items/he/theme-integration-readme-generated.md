---
כדי לצרף תגובות לכל פוסט כפי שהתמיכה המובנית של Hugo ב-Disqus עושה, הוסיפו שורה אחת לתבנית single של ערכת הנושא שלכם (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

ה-partial יוצג רק כאשר `tenantId` מוגדר. השביתו תגובות בעמוד בודד באמצעות front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---