---
Legen Sie Ihre Tenant-ID einmal in `hugo.toml` fest:

```toml
[params.fastcomments]
  tenantId = "demo"   # ersetzen Sie "demo" durch Ihre FastComments-Tenant-ID
```

Dann binden Sie entweder das Kommentar-Widget in Ihr Theme ein (siehe [Theme Integration](#theme-integration-readme-generated)), oder fügen Sie einen Shortcode in das Markdown einer beliebigen Seite ein:

```text
\{{< fastcomments >}}
```
---