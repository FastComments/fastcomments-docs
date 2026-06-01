---
Задайте своя tenant ID веднъж в `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # заменете "demo" с вашия FastComments tenant ID
```

След това или интегрирайте уиджета за коментари в темата си (виж [Интеграция на темата](#theme-integration-readme-generated)), или поставете shortcode в Markdown-а на която и да е страница:

```text
\{{< fastcomments >}}
```
---