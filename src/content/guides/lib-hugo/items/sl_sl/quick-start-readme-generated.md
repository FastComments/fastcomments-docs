---
Nastavite svoj ID najemnika enkrat v `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # zamenjajte "demo" z ID-jem najemnika FastComments
```

Nato bodisi vključite pripomoček za komentarje v svojo temo (glejte [Integracija teme](#theme-integration-readme-generated)), ali vstavite shortcode v Markdown poljubne strani:

```text
\{{< fastcomments >}}
```
---