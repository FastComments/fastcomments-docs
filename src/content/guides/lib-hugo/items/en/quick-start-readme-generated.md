Set your tenant ID once in `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # replace "demo" with your FastComments tenant ID
```

Then either wire the comments widget into your theme (see [Theme Integration](#theme-integration-readme-generated)), or drop a shortcode into any page's Markdown:

```text
\{{< fastcomments >}}
```