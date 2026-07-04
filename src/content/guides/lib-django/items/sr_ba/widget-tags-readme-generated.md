Svaki vidžet ima svoju oznaku. Svi oni prihvataju `**extra` ključne argumente, koji se spajaju u konfiguraciju vidžeta takvom kakvom je (koristite camelCase ključeve) za sve što nije pokriveno imenovanim argumentima ispod.

| Oznaka | Vidžet |
|---|---|
| `{% fastcomments %}` | Komentari |
| `{% fastcomments_live_chat %}` | Živi chat |
| `{% fastcomments_comment_count %}` | Značka broja komentara |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Grupni brojači komentara |
| `{% fastcomments_collab_chat target="#el" %}` | Kolaborativni (inline) chat |
| `{% fastcomments_image_chat target="#el" %}` | Chat za anotaciju slika |
| `{% fastcomments_recent_comments %}` | Nedavni komentari |
| `{% fastcomments_recent_discussions %}` | Nedavne rasprave |
| `{% fastcomments_reviews_summary %}` | Sažetak recenzija |
| `{% fastcomments_top_pages %}` | Najviše diskutovane stranice |
| `{% fastcomments_user_activity user_id="..." %}` | Tok aktivnosti korisnika |

Imenovani argumenti mapiraju na camelCase konfiguracijske ključeve vidžeta:

| Argument | Config key | Oznake |
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

Primjeri:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Komentari: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Kolaborativni chat se prilaže postojećem elementu na stranici #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Grupni brojači: postavite markere, a zatim jedan grupni učitač popuni sve #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```