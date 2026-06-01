Все параметры виджета FastComments задаются в `[params.fastcomments]` в `hugo.toml`, и могут быть переопределены для каждой страницы во front matter под `[fastcomments]`. Приоритет (от низшего к высшему): параметры сайта, front matter страницы, параметры шорткода.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter страницы
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Если не указаны ни `url`, ни `urlId`, то `url` по умолчанию устанавливается в permalink страницы, чтобы ветки комментариев оставались привязанными к стабильному URL.

### Хранение данных в ЕС

Клиенты из ЕС устанавливают `region = "eu"`, чтобы направлять виджет на `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Примечание о регистре ключей

Hugo приводит все ключи к нижнему регистру в `hugo.toml` и во front matter, но виджеты FastComments требуют ключи в camelCase (`tenantId`, `hasDarkBackground`). Этот компонент автоматически восстанавливает правильный регистр для каждого известного верхнеуровневого параметра, поэтому указывайте опции в их обычной camelCase форме. Ключи, вложенные внутрь значения объекта (например, ключи карты `translations` или поля `pageReactConfig`), не восстанавливаются. Настраивайте их через интерфейс кастомизации панели управления FastComments.