Every widget has its own tag. All of them accept `**extra` keyword arguments,
which are merged into the widget config as-is (use camelCase keys) for anything
not covered by the named arguments below.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Комментарии |
| `{% fastcomments_live_chat %}` | Живой чат |
| `{% fastcomments_comment_count %}` | Бейдж количества комментариев |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Суммарные подсчёты комментариев |
| `{% fastcomments_collab_chat target="#el" %}` | Совместный (встроенный) чат |
| `{% fastcomments_image_chat target="#el" %}` | Чат с аннотацией изображений |
| `{% fastcomments_recent_comments %}` | Последние комментарии |
| `{% fastcomments_recent_discussions %}` | Последние обсуждения |
| `{% fastcomments_reviews_summary %}` | Сводка отзывов |
| `{% fastcomments_top_pages %}` | Самые обсуждаемые страницы |
| `{% fastcomments_user_activity user_id="..." %}` | Лента активности пользователя |

Именованные аргументы соответствуют camelCase ключам конфигурации виджета:

| Argument | Config key | Tags |
|---|---|---|
| `url_id` | `urlId` | комментарии, живой чат, бейдж количества комментариев, совместный/чат с изображениями, последние комментарии, сводка отзывов |
| `url` | `url` | комментарии, живой чат, совместный/чат с изображениями |
| `readonly` | `readonly` | комментарии, живой чат, совместный/чат с изображениями |
| `locale` | `locale` | комментарии, живой чат, совместный/чат с изображениями, активность пользователя |
| `has_dark_background` | `hasDarkBackground` | все |
| `default_sort_direction` | `defaultSortDirection` | комментарии, живой чат, совместный/чат с изображениями |
| `number_only` | `numberOnly` | бейдж количества комментариев |
| `is_live` | `isLive` | бейдж количества комментариев |
| `count` | `count` | последние комментарии, последние обсуждения |
| `target` | (querySelector, not sent) | совместный чат, чат с изображениями |
| `chat_square_percentage` | `chatSquarePercentage` | чат с изображениями |
| `user_id` | `userId` | активность пользователя |

Примеры:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Комментарии: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Совместный чат привязывается к существующему элементу на странице #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Суммарные подсчёты: разместите маркеры, затем один bulk loader заполнит их все #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```