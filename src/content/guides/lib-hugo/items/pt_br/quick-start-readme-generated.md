Defina seu tenant ID uma vez em `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # substitua "demo" pelo seu FastComments tenant ID
```

Em seguida, ou integre o widget de comentários ao seu tema (veja [Integração do Tema](#theme-integration-readme-generated)), ou insira um shortcode no Markdown de qualquer página:

```text
\{{< fastcomments >}}
```