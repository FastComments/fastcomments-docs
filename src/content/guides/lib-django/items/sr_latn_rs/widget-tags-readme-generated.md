Svaki vidžet ima svoj tag. Svi oni prihvataju `**extra` ključne argumente, koji se spajaju u konfiguraciju vidžeta takvu kakvu jeste (koristite camelCase ključeve) za sve što nije pokriveno imenovanim argumentima ispod.

| Oznaka | Vidžet |
|---|---|
| `{% fastcomments %}` | Komentari |
| `{% fastcomments_live_chat %}` | Živi razgovor |
| `{% fastcomments_comment_count %}` | Značka broja komentara |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Skupni brojevi komentara |
| `{% fastcomments_collab_chat target="#el" %}` | Kolaborativni (ugrađeni) chat |
| `{% fastcomments_image_chat target="#el" %}` | Chat za anotaciju slika |
| `{% fastcomments_recent_comments %}` | Nedavni komentari |
| `{% fastcomments_recent_discussions %}` | Nedavne diskusije |
| `{% fastcomments_reviews_summary %}` | Sažetak recenzija |
| `{% fastcomments_top_pages %}` | Najviše diskutovane stranice |
| `{% fastcomments_user_activity user_id="..." %}` | Tok aktivnosti korisnika |

Imenovani argumenti mapiraju na camelCase ključeve konfiguracije vidžeta:

| Argument | Ključ konfiguracije | Tagovi |
|---|---|---|
| `url_id` | `urlId` | komentari, živi razgovor, broj komentara, kolaborativni/chat za slike, nedavni komentari, sažetak recenzija |
| `url` | `url` | komentari, živi razgovor, kolaborativni/chat za slike |
| `readonly` | `readonly` | komentari, živi razgovor, kolaborativni/chat za slike |
| `locale` | `locale` | komentari, živi razgovor, kolaborativni/chat za slike, aktivnost korisnika |
| `has_dark_background` | `hasDarkBackground` | sve |
| `default_sort_direction` | `defaultSortDirection` | komentari, živi razgovor, kolaborativni/chat za slike |
| `number_only` | `numberOnly` | broj komentara |
| `is_live` | `isLive` | broj komentara |
| `count` | `count` | nedavni komentari, nedavne diskusije |
| `target` | (querySelector, not sent) | kolaborativni chat, chat za slike |
| `chat_square_percentage` | `chatSquarePercentage` | chat za slike |
| `user_id` | `userId` | aktivnost korisnika |

Primeri:

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
---