Всі параметри віджету FastComments задаються в `[params.fastcomments]` у `hugo.toml`, і можуть бути перевизначені для кожної сторінки у front matter під `[fastcomments]`. Пріоритет, від найнижчого до найвищого: параметри сайту, front matter сторінки, параметри шорткоду.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter сторінки
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

When neither `url` nor `urlId` is provided, `url` defaults to the page's permalink so comment threads stay tied to a stable URL.

### Розміщення даних у ЄС

Клієнти з ЄС вказують `region = "eu"`, щоб направити віджет на `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Примітка щодо регістру ключів

Hugo приводить усі ключі до нижнього регістру в `hugo.toml` та front matter, але віджетам FastComments потрібні ключі в camelCase (`tenantId`, `hasDarkBackground`). Цей компонент автоматично відновлює правильний регістр для кожної відомої верхньорівневої опції, тож записуйте опції у звичному camelCase. Ключі, вкладені в значення об'єкта (наприклад, ключі мапи `translations`, або поля `pageReactConfig`), не відновлюються. Налаштовуйте їх через інтерфейс налаштування панелі FastComments.