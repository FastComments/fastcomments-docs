Встановіть свій tenant ID один раз у `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # замініть "demo" на ваш FastComments tenant ID
```

Потім або підключіть віджет коментарів до вашої теми (див. [Інтеграція теми](#theme-integration-readme-generated)), або вставте шорткод у Markdown будь-якої сторінки:

```text
\{{< fastcomments >}}
```