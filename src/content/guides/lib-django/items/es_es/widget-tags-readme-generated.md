Cada widget tiene su propia etiqueta. Todos aceptan argumentos de palabra clave `**extra`, los cuales se fusionan en la configuración del widget tal cual (use claves camelCase) para cualquier cosa que no esté cubierta por los argumentos nombrados a continuación.

| Etiqueta | Widget |
|---|---|
| `{% fastcomments %}` | Comments |
| `{% fastcomments_live_chat %}` | Live chat |
| `{% fastcomments_comment_count %}` | Comment count badge |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Bulk comment counts |
| `{% fastcomments_collab_chat target="#el" %}` | Collaborative (inline) chat |
| `{% fastcomments_image_chat target="#el" %}` | Image annotation chat |
| `{% fastcomments_recent_comments %}` | Recent comments |
| `{% fastcomments_recent_discussions %}` | Recent discussions |
| `{% fastcomments_reviews_summary %}` | Reviews summary |
| `{% fastcomments_top_pages %}` | Most-discussed pages |
| `{% fastcomments_user_activity user_id="..." %}` | A user's activity feed |

Los argumentos nombrados se asignan a las claves camelCase de la configuración del widget:

| Argumento | Clave de configuración | Etiquetas |
|---|---|---|
| `url_id` | `urlId` | comments, live chat, comment count, collab/image chat, recent comments, reviews summary |
| `url` | `url` | comments, live chat, collab/image chat |
| `readonly` | `readonly` | comments, live chat, collab/image chat |
| `locale` | `locale` | comments, live chat, collab/image chat, user activity |
| `has_dark_background` | `hasDarkBackground` | all |
| `default_sort_direction` | `defaultSortDirection` | comments, live chat, collab/image chat |
| `number_only` | `numberOnly` | comment count |
| `is_live` | `isLive` | comment count |
| `count` | `count` | recent comments, recent discussions |
| `target` | (querySelector, not sent) | collab chat, image chat |
| `chat_square_percentage` | `chatSquarePercentage` | image chat |
| `user_id` | `userId` | user activity |

Ejemplos:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# El chat colaborativo se adjunta a un elemento existente en la página #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Recuentos masivos: coloque marcadores, luego un cargador masivo los completa todos #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```
---