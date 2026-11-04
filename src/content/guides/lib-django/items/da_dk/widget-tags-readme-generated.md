Hver widget har sit eget tag. Alle accepterer `**extra` keyword-argumenter,
som flettes direkte ind i widget-konfigurationen (brug camelCase-nøgler) for alt,
der ikke er dækket af de navngivne argumenter nedenfor.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Kommentarer |
| `{% fastcomments_live_chat %}` | Live chat |
| `{% fastcomments_comment_count %}` | Kommentarantal-badge |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Samlede kommentarantal |
| `{% fastcomments_collab_chat target="#el" %}` | Samarbejdschat (inline) |
| `{% fastcomments_image_chat target="#el" %}` | Chat til billedannotering |
| `{% fastcomments_recent_comments %}` | Seneste kommentarer |
| `{% fastcomments_recent_discussions %}` | Seneste diskussioner |
| `{% fastcomments_reviews_summary %}` | Anmeldelsesoversigt |
| `{% fastcomments_top_pages %}` | Mest diskuterede sider |
| `{% fastcomments_user_activity user_id="..." %}` | En brugers aktivitetsfeed |

Navngivne argumenter mapper til widgettens camelCase-konfigurationsnøgler:

| Argument | Konfigurationsnøgle | Tags |
|---|---|---|
| `url_id` | `urlId` | comments, live chat, comment count, collab/image chat, recent comments, reviews summary |
| `url` | `url` | comments, live chat, collab/image chat |
| `readonly` | `readonly` | comments, live chat, collab/image chat |
| `locale` | `locale` | comments, live chat, collab/image chat, user activity |
| `has_dark_background` | `hasDarkBackground` | alle |
| `default_sort_direction` | `defaultSortDirection` | comments, live chat, collab/image chat |
| `number_only` | `numberOnly` | comment count |
| `is_live` | `isLive` | comment count |
| `count` | `count` | recent comments, recent discussions |
| `target` | (querySelector, sendes ikke) | collab chat, image chat |
| `chat_square_percentage` | `chatSquarePercentage` | image chat |
| `user_id` | `userId` | user activity |

Eksempler:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Collab chat knytter sig til et eksisterende element på siden #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Samlede antal: placér markører, og én bulk-loader udfylder dem alle #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```