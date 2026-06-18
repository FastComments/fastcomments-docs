---
Podesite svoj tenant ID jednom u `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # zamenite "demo" sa vašim FastComments tenant ID-om
```

Zatim povežite widget za komentare sa svojom temom (pogledajte [Integracija teme](#theme-integration-readme-generated)), ili ubacite shortcode u Markdown bilo koje stranice:

```text
\{{< fastcomments >}}
```
---