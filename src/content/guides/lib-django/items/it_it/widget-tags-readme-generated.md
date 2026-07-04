Ogni widget ha il proprio tag. Tutti accettano argomenti chiave `**extra`, che vengono fusi nella configurazione del widget così com'è (usa chiavi camelCase) per tutto ciò che non è coperto dagli argomenti nominati di seguito.

| Tag | Widget |
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

Gli argomenti nominati corrispondono alle chiavi di configurazione camelCase del widget:

| Argument | Config key | Tags |
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

Esempi:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# La chat collaborativa si collega a un elemento esistente nella pagina #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Conteggi in blocco: posiziona i marker, poi un loader bulk li riempie tutti #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```