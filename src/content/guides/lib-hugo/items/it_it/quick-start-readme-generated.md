---
Imposta il tuo tenant ID una volta in `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # sostituire "demo" con il tuo tenant ID FastComments
```

Poi collega il widget dei commenti al tuo tema (vedi [Integrazione del tema](#theme-integration-readme-generated)), oppure inserisci uno shortcode nel Markdown di qualsiasi pagina:

```text
\{{< fastcomments >}}
```
---