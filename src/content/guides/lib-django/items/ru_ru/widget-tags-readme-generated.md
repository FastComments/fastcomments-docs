У каждого виджета есть свой тег. Все они принимают именованные аргументы `**extra`,
которые добавляются в конфигурацию виджета как есть (используйте ключи в camelCase)
для всего, что не покрывается именованными аргументами ниже.

| Тег | Виджет |
|---|---|
| `{% fastcomments %}` | Комментарии |
| `{% fastcomments_live_chat %}` | Живой чат |
| `{% fastcomments_comment_count %}` | Значок количества комментариев |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Массовый подсчёт комментариев |
| `{% fastcomments_collab_chat target="#el" %}` | Совместный (встроенный) чат |
| `{% fastcomments_image_chat target="#el" %}` | Чат с аннотациями к изображениям |
| `{% fastcomments_recent_comments %}` | Последние комментарии |
| `{% fastcomments_recent_discussions %}` | Последние обсуждения |
| `{% fastcomments_reviews_summary %}` | Сводка отзывов |
| `{% fastcomments_top_pages %}` | Самые обсуждаемые страницы |
| `{% fastcomments_user_activity user_id="..." %}` | Лента активности пользователя |

Именованные аргументы соответствуют ключам конфигурации виджета в camelCase:

| Аргумент | Ключ конфигурации | Теги |
|---|---|---|
| `url_id` | `urlId` | комментарии, живой чат, количество комментариев, совместный чат/чат изображений, последние комментарии, сводка отзывов |
| `url` | `url` | комментарии, живой чат, совместный чат/чат изображений |
| `readonly` | `readonly` | комментарии, живой чат, совместный чат/чат изображений |
| `locale` | `locale` | комментарии, живой чат, совместный чат/чат изображений, активность пользователя |
| `has_dark_background` | `hasDarkBackground` | все |
| `default_sort_direction` | `defaultSortDirection` | комментарии, живой чат, совместный чат/чат изображений |
| `number_only` | `numberOnly` | количество комментариев |
| `is_live` | `isLive` | количество комментариев |
| `count` | `count` | последние комментарии, последние обсуждения |
| `target` | (querySelector, не отправляется) | совместный чат, чат изображений |
| `chat_square_percentage` | `chatSquarePercentage` | чат изображений |
| `user_id` | `userId` | активность пользователя |

Примеры:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Совместный чат привязывается к существующему элементу на странице #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Массовый подсчёт: расставьте маркеры, затем один массовый загрузчик заполнит их все #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```