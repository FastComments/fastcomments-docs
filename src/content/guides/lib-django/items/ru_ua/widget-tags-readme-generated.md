Кожен віджет має свій тег. Усі вони приймають аргументи `**extra`, які без змін (через camelCase) додаються до конфігурації віджета для всього, що не охоплено іменованими аргументами нижче.

| Тег | Віджет |
|---|---|
| `{% fastcomments %}` | Коментарі |
| `{% fastcomments_live_chat %}` | Живий чат |
| `{% fastcomments_comment_count %}` | Показник кількості коментарів |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Масові підрахунки коментарів |
| `{% fastcomments_collab_chat target="#el" %}` | Спільний (вбудований) чат |
| `{% fastcomments_image_chat target="#el" %}` | Чат з анотацією зображень |
| `{% fastcomments_recent_comments %}` | Останні коментарі |
| `{% fastcomments_recent_discussions %}` | Останні дискусії |
| `{% fastcomments_reviews_summary %}` | Підсумок відгуків |
| `{% fastcomments_top_pages %}` | Найбільш обговорювані сторінки |
| `{% fastcomments_user_activity user_id="..." %}` | Стрічка активності користувача |

Іменовані аргументи відповідають camelCase‑ключам конфігурації віджета:

| Аргумент | Ключ конфігурації | Теги |
|---|---|---|
| `url_id` | `urlId` | коментарі, живий чат, підрахунок коментарів, спільний/зображення чат, останні коментарі, підсумок відгуків |
| `url` | `url` | коментарі, живий чат, спільний/зображення чат |
| `readonly` | `readonly` | коментарі, живий чат, спільний/зображення чат |
| `locale` | `locale` | коментарі, живий чат, спільний/зображення чат, активність користувача |
| `has_dark_background` | `hasDarkBackground` | всі |
| `default_sort_direction` | `defaultSortDirection` | коментарі, живий чат, спільний/зображення чат |
| `number_only` | `numberOnly` | підрахунок коментарів |
| `is_live` | `isLive` | підрахунок коментарів |
| `count` | `count` | останні коментарі, останні дискусії |
| `target` | (querySelector, не передається) | спільний чат, чат з анотацією зображень |
| `chat_square_percentage` | `chatSquarePercentage` | чат з анотацією зображень |
| `user_id` | `userId` | активність користувача |

Приклади:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Collab chat attaches to an existing element on the page #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Bulk counts: place markers, then one bulk loader fills them all in #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```