---
הגדר את מזהה ה-tenant שלך פעם אחת בקובץ `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # החלף את "demo" במזהה ה-tenant של FastComments שלך
```

Then either wire the comments widget into your theme (see [אינטגרציה של התמה](#theme-integration-readme-generated)), or drop a shortcode into any page's Markdown:

```text
\{{< fastcomments >}}
```
---