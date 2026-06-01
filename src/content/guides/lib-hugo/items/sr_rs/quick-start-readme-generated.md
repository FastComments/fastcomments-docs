---
Подесите ваш tenant ID једном у `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # замените "demo" вашим FastComments tenant ID-ом
```

Затим или повежите видгет коментара у вашу тему (види [Интеграција теме](#theme-integration-readme-generated)), или убаците shortcode у било коју страницу у Markdown:

```text
\{{< fastcomments >}}
```
---