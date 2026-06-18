Все параметры виджета FastComments задаются в `[params.fastcomments]` в `hugo.toml` и могут быть переопределены для каждой страницы во front matter под `[fastcomments]`. Приоритет, от низшего к высшему: параметры сайта, front matter страницы, параметры шорткода.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Если не указаны ни `url`, ни `urlId`, то `url` по умолчанию принимает значение permalink страницы, чтобы потоки комментариев оставались привязанными к стабильному URL.

### Размещение данных в ЕС

Клиенты из ЕС указывают `region = "eu"`, чтобы маршрут виджета вел на `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Примечание о регистре ключей

Hugo приводит к нижнему регистру каждый ключ в `hugo.toml` и front matter, но виджеты FastComments требуют ключей в camelCase (`tenantId`, `hasDarkBackground`). Этот компонент автоматически восстанавливает правильный регистр для каждой известной опции верхнего уровня, поэтому записывайте опции в их обычной форме camelCase. Ключи, вложенные внутри значения объекта (например, ключи карты `translations` или поля `pageReactConfig`), не восстанавливаются. Настраивайте их через UI кастомизации в панели управления FastComments.