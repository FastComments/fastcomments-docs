---
Ustaw identyfikator najemcy raz w `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # zamień "demo" na identyfikator najemcy FastComments
```

Następnie albo podłącz widget komentarzy do motywu (zobacz [Integracja motywu](#theme-integration-readme-generated)), albo wstaw shortcode na dowolnej stronie w Markdown:

```text
\{{< fastcomments >}}
```
---