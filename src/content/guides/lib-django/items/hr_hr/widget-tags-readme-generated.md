Svaki widget ima svoju oznaku. Sve one prihvaćaju `**extra` ključne argumente,
koji se spajaju u konfiguraciju widgeta onakvima kakvi jesu (koristite camelCase ključeve) za sve
što nije pokriveno imenovanim argumentima u nastavku.

| Oznaka | Widget |
|---|---|
| `{% fastcomments %}` | Komentari |
| `{% fastcomments_live_chat %}` | Live chat |
| `{% fastcomments_comment_count %}` | Bedž broja komentara |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Brojači komentara u grupi |
| `{% fastcomments_collab_chat target="#el" %}` | Suradnički (inline) chat |
| `{% fastcomments_image_chat target="#el" %}` | Chat s bilješkama na slikama |
| `{% fastcomments_recent_comments %}` | Nedavni komentari |
| `{% fastcomments_recent_discussions %}` | Nedavne rasprave |
| `{% fastcomments_reviews_summary %}` | Sažetak recenzija |
| `{% fastcomments_top_pages %}` | Najraspravljanije stranice |
| `{% fastcomments_user_activity user_id="..." %}` | Feed aktivnosti korisnika |

Imenovani argumenti mapiraju se na camelCase ključeve konfiguracije widgeta:

| Argument | Ključ konfiguracije | Oznake |
|---|---|---|
| `url_id` | `urlId` | komentari, live chat, brojač komentara, collab/image chat, nedavni komentari, sažetak recenzija |
| `url` | `url` | komentari, live chat, collab/image chat |
| `readonly` | `readonly` | komentari, live chat, collab/image chat |
| `locale` | `locale` | komentari, live chat, collab/image chat, aktivnost korisnika |
| `has_dark_background` | `hasDarkBackground` | sve |
| `default_sort_direction` | `defaultSortDirection` | komentari, live chat, collab/image chat |
| `number_only` | `numberOnly` | brojač komentara |
| `is_live` | `isLive` | brojač komentara |
| `count` | `count` | nedavni komentari, nedavne rasprave |
| `target` | (querySelector, ne šalje se) | collab chat, image chat |
| `chat_square_percentage` | `chatSquarePercentage` | image chat |
| `user_id` | `userId` | aktivnost korisnika |

Primjeri:

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