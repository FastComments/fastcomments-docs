---
Stel uw tenant ID eenmalig in in `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # vervang "demo" door uw FastComments tenant ID
```

Verbind daarna ofwel de reacties-widget met uw thema (zie [Thema-integratie](#theme-integration-readme-generated)), of voeg een shortcode toe aan de Markdown van een pagina:

```text
\{{< fastcomments >}}
```
---