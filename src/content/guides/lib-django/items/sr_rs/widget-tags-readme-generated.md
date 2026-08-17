Сваки виџет има своју ознаку. Сви они прихватају `**extra` кључне аргументе,
који се спајају у конфигурацију виџета без измена (користите camelCase кључеве) за све
што није обухваћено именованим аргументима испод.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Коментари |
| `{% fastcomments_live_chat %}` | Уживо ћаскање |
| `{% fastcomments_comment_count %}` | Ознака броја коментара |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Масовни број коментара |
| `{% fastcomments_collab_chat target="#el" %}` | Колаборативно (уграђено) ћаскање |
| `{% fastcomments_image_chat target="#el" %}` | ћаскање за анотацију слика |
| `{% fastcomments_recent_comments %}` | Скорашњи коментари |
| `{% fastcomments_recent_discussions %}` | Скорашње дискусије |
| `{% fastcomments_reviews_summary %}` | Сажетак рецензија |
| `{% fastcomments_top_pages %}` | Најдискусијније странице |
| `{% fastcomments_user_activity user_id="..." %}` | Фид активности корисника |

Именовани аргументи мапирају се на camelCase кључеве конфигурације виџета:

| Argument | Config key | Tags |
|---|---|---|
| `url_id` | `urlId` | коментари, уживо ћаскање, број коментара, колаб/слика ћаскање, скорашњи коментари, сажетак рецензија |
| `url` | `url` | коментари, уживо ћаскање, колаб/слика ћаскање |
| `readonly` | `readonly` | коментари, уживо ћаскање, колаб/слика ћаскање |
| `locale` | `locale` | коментари, уживо ћаскање, колаб/слика ћаскање, активност корисника |
| `has_dark_background` | `hasDarkBackground` | сви |
| `default_sort_direction` | `defaultSortDirection` | коментари, уживо ћаскање, колаб/слика ћаскање |
| `number_only` | `numberOnly` | број коментара |
| `is_live` | `isLive` | број коментара |
| `count` | `count` | скорашњи коментари, скорашње дискусије |
| `target` | (querySelector, not sent) | колаб ћаскање, слика ћаскање |
| `chat_square_percentage` | `chatSquarePercentage` | слика ћаскање |
| `user_id` | `userId` | активност корисника |

Examples:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Коментари: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Колаб ћаскање се прикаче на постојећи елемент на страници #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Масовни бројеви: поставите маркере, затим један масовни учитавач их све попуњава #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```