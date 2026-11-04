Усі параметри віджета FastComments задаються у `[params.fastcomments]` у файлі `hugo.toml`, і їх можна перевизначати на рівні окремої сторінки у front matter у розділі `[fastcomments]`. Пріоритет від найнижчого до найвищого: параметри сайту, front matter сторінки, параметри шорткоду.

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

Коли не вказано ані `url`, ані `urlId`, параметр `url` за замовчуванням отримує постійне посилання сторінки, щоб гілки коментарів залишалися прив'язаними до стабільного URL.

### Розміщення даних у ЄС

Клієнти з ЄС встановлюють `region = "eu"`, щоб спрямувати віджет на `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Примітка щодо регістру ключів

Hugo перетворює всі ключі у `hugo.toml` та front matter на нижній регістр, але віджети FastComments вимагають ключів у camelCase (`tenantId`, `hasDarkBackground`). Цей компонент автоматично відновлює правильний регістр для всіх відомих параметрів верхнього рівня, тому параметри можна записувати у звичайній формі camelCase. Ключі, вкладені всередину об'єктних значень (наприклад, ключі мапи `translations` або поля `pageReactConfig`), не відновлюються. Налаштовуйте їх через інтерфейс кастомізації у панелі керування FastComments.