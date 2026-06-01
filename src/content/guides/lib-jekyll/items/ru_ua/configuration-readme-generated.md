Конфигурация приходит из трёх источников. Побеждает более поздний источник:

1. **Глобальные значения по умолчанию** в `_config.yml` под ключом `fastcomments:`.
2. **Контекст страницы**, выводимый автоматически для виджетов, привязанных к странице (см. ниже).
3. **Атрибуты тега**, указанные в самом теге.

Таким образом, `url_id` в теге переопределяет значение, полученное со страницы, которое, в свою очередь, переопределяет любой глобальный дефолт.

### Синтаксис атрибутов

Атрибуты — это пары `key=value` в `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **В кавычках** значения (`"..."` или `'...'`) — буквальные строки.
- **Без кавычек** `true`/`false` становятся булевыми, а числа — числами.
- **Без кавычек** всё остальное разрешается как переменная Liquid из контекста страницы, например `url_id=page.slug`. (Liquid не разворачивает `{% raw %}\{{ ... }}{% endraw %}` внутри атрибутов тега, поэтому используйте простую форму `page.slug`, а не `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Ключи атрибутов и конфигурации в snake_case автоматически маппятся на camelCase-ключи, которые ожидает FastComments (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, и т.д.). Любая другая опция из [настроек виджета](https://docs.fastcomments.com/guide-customizations-and-configuration.html) передаётся напрямую таким же образом.

### Значения, получаемые со страницы

Для виджетов, привязанных к странице (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) эти значения заполняются автоматически из текущей страницы, если вы сами их не укажете:

- `url_id` ← `page.url` (стабильный идентификатор, независимый от домена посетителя)
- `url` ← `site.url` + `page.url` (только когда `url` установлен в `_config.yml`)
- `page_title` ← `page.title`

Виджеты уровня сайта (recent comments/discussions, top pages, reviews summary, user activity feed, bulk count) не привязаны к странице и не получают эти значения.

### Локация хранения данных в ЕС

Клиенты из ЕС добавляют `region: eu`, либо глобально:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

или для каждого тега: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Виджеты тогда загружаются с CDN ЕС.