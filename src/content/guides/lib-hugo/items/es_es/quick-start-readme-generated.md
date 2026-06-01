---
Establece tu tenant ID una vez en `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # reemplaza "demo" por tu FastComments tenant ID
```

Then either wire the comments widget into your theme (see [Integración del tema](#theme-integration-readme-generated)), or drop a shortcode into any page's Markdown:

```text
\{{< fastcomments >}}
```
---