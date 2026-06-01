---
Podesite svoj tenant ID jednom u `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # zamenite "demo" svojim FastComments tenant ID-jem
```

Zatim ili povežite comments widget u svoju temu (pogledajte [Integracija teme](#theme-integration-readme-generated)), ili ubacite shortcode u Markdown bilo koje stranice:

```text
\{{< fastcomments >}}
```
---