Angiv dit tenant-id én gang i `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # erstat "demo" med dit FastComments tenant-id
```

Tilslut derefter kommentarfunktionen til dit tema (se [Tema-integration](#theme-integration-readme-generated)), eller indsæt en shortcode i en sides Markdown:

```text
\{{< fastcomments >}}
```