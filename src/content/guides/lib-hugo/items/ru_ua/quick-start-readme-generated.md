---
Установите ваш tenant ID один раз в `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # замените "demo" на ваш FastComments tenant ID
```

Затем либо подключите виджет комментариев к вашей теме (см. [Интеграция темы](#theme-integration-readme-generated)), либо вставьте шорткод в Markdown любой страницы:

```text
\{{< fastcomments >}}
```
---