---
Всички опции на FastComments widget-а се задават под `[params.fastcomments]` в `hugo.toml`, и могат да бъдат преопределени за всяка страница във front matter под `[fastcomments]`. Предпочитание, от ниско към високо: site params, page front matter, shortcode parameters.

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

Когато нито `url`, нито `urlId` са зададени, `url` по подразбиране е permalink на страницата, така че нишките с коментари да останат свързани с постоянен URL.

### Съхранение на данни в ЕС

Клиентите от ЕС задават `region = "eu"`, за да пренасочат widget-а към `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Забележка относно регистра на ключовете

Hugo прави всички ключове с малки букви в `hugo.toml` и във front matter, но FastComments widgets изискват camelCase ключове (`tenantId`, `hasDarkBackground`). Този компонент автоматично възстановява правилния регистър за всяка известна опция от най-високо ниво, затова задавайте опциите в тяхната нормална camelCase форма. Ключовете, вложени в стойност на обект (например ключовете на `translations` map или полетата на `pageReactConfig`), не се възстановяват. Конфигурирайте тези чрез потребителския интерфейс за персонализиране на таблото за управление на FastComments вместо това.
---