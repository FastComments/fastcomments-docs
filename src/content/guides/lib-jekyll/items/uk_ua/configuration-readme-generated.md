Конфігурація надходить з трьох місць. Перевага належить пізнішим джерелам:

1. **Глобальні значення за замовчуванням** у `_config.yml` під ключем `fastcomments:`.
2. **Контекст сторінки**, який автоматично визначається для віджетів, прив'язаних до сторінки (див. нижче).
3. **Атрибути тега**, вказані безпосередньо в самому тегі.

Отже, `url_id`, вказаний у тегі, перекриває значення, отримане зі сторінки, яке, у свою чергу, перекриває будь-яке глобальне значення за замовчуванням.

### Attribute syntax

Атрибути — це пари `key=value` у `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Quoted** values (`"..."` or `'...'`) are literal strings.
- **Unquoted** `true`/`false` become booleans, and numbers become numbers.
- **Unquoted** anything else is resolved as a Liquid variable from the page context, e.g.
  `url_id=page.slug`. (Liquid does not expand `{% raw %}\{{ ... }}{% endraw %}` inside a tag's
  attributes, so use the bare `page.slug` form rather than `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Ключі атрибутів і конфігурації в snake_case автоматично відображаються на camelCase ключі, які очікує FastComments (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground` тощо). Будь-яка інша опція з [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) проходить без змін тим самим чином.

### Page-derived values

Для віджетів, прив'язаних до сторінки (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`), ці значення заповнюються автоматично зі поточної сторінки, якщо ви не встановите їх самі:

- `url_id` ← `page.url` (стабільний ідентифікатор, незалежний від домену відвідувача)
- `url` ← `site.url` + `page.url` (лише коли `url` встановлено в `_config.yml`)
- `page_title` ← `page.title`

Сайтові віджети (недавні коментарі/обговорення, топ-сторінки, зведення відгуків, стрічка активності користувачів, масові підрахунки) не прив'язані до сторінки і не заповнюють ці значення.

### EU data residency

Клієнти з ЄС додають `region: eu`, або глобально:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

або для окремого тега: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Віджети тоді завантажуються з CDN у ЄС.