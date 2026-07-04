Every widget has its own tag. All of them accept `**extra` keyword arguments,
which are merged into the widget config as‑is (use camelCase keys) for anything
not covered by the named arguments below.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Komentari |
| `{% fastcomments_live_chat %}` | Uživo čet |
| `{% fastcomments_comment_count %}` | Značka broja komentara |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Skupni brojevi komentara |
| `{% fastcomments_collab_chat target="#el" %}` | Kolaborativni (inline) čet |
| `{% fastcomments_image_chat target="#el" %}` | Čet za anotaciju slika |
| `{% fastcomments_recent_comments %}` | Nedavni komentari |
| `{% fastcomments_recent_discussions %}` | Nedavne diskusije |
| `{% fastcomments_reviews_summary %}` | Pregled recenzija |
| `{% fastcomments_top_pages %}` | Najviše diskutovane stranice |
| `{% fastcomments_user_activity user_id="..." %}` | Tok aktivnosti korisnika |

Named arguments map to the widget's camelCase config keys:

| Argument | Config key | Tags |
|---|---|---|
| `url_id` | `urlId` | komentari, uživo čet, broj komentara, kolaborativni/čet za slike, nedavni komentari, pregled recenzija |
| `url` | `url` | komentari, uživo čet, kolaborativni/čet za slike |
| `readonly` | `readonly` | komentari, uživo čet, kolaborativni/čet za slike |
| `locale` | `locale` | komentari, uživo čet, kolaborativni/čet za slike, aktivnost korisnika |
| `has_dark_background` | `hasDarkBackground` | sve |
| `default_sort_direction` | `defaultSortDirection` | komentari, uživo čet, kolaborativni/čet za slike |
| `number_only` | `numberOnly` | broj komentara |
| `is_live` | `isLive` | broj komentara |
| `count` | `count` | nedavni komentari, nedavne diskusije |
| `target` | (querySelector, not sent) | kolaborativni čet, čet za slike |
| `chat_square_percentage` | `chatSquarePercentage` | čet za slike |
| `user_id` | `userId` | aktivnost korisnika |

Examples:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Kolaborativni čet se prikači na postojeći element na stranici #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Skupni brojevi: postavite markere, zatim jedan skupni učitač popuni sve #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```