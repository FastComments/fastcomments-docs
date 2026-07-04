Jedes Widget hat sein eigenes Tag. Alle akzeptieren das Schlüsselwort‑Argument `**extra`, das unverändert (mit camelCase‑Schlüsseln) in die Widget‑Konfiguration übernommen wird, wenn es nicht von den unten genannten benannten Argumenten abgedeckt wird.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Kommentare |
| `{% fastcomments_live_chat %}` | Live‑Chat |
| `{% fastcomments_comment_count %}` | Kommentar‑Zähler‑Badge |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Bulk‑Kommentarzähler |
| `{% fastcomments_collab_chat target="#el" %}` | Kollaborativer (inline) Chat |
| `{% fastcomments_image_chat target="#el" %}` | Bild‑Annotierungs‑Chat |
| `{% fastcomments_recent_comments %}` | Aktuelle Kommentare |
| `{% fastcomments_recent_discussions %}` | Aktuelle Diskussionen |
| `{% fastcomments_reviews_summary %}` | Zusammenfassung von Bewertungen |
| `{% fastcomments_top_pages %}` | Meist‑diskutierte Seiten |
| `{% fastcomments_user_activity user_id="..." %}` | Aktivitäts‑Feed eines Benutzers |

Benannte Argumente entsprechen den camelCase‑Konfigurationsschlüsseln des Widgets:

| Argument | Konfigurationsschlüssel | Tags |
|---|---|---|
| `url_id` | `urlId` | Kommentare, Live‑Chat, Kommentar‑Zähler, Kollaborativer/Bild‑Chat, aktuelle Kommentare, Zusammenfassung von Bewertungen |
| `url` | `url` | Kommentare, Live‑Chat, Kollaborativer/Bild‑Chat |
| `readonly` | `readonly` | Kommentare, Live‑Chat, Kollaborativer/Bild‑Chat |
| `locale` | `locale` | Kommentare, Live‑Chat, Kollaborativer/Bild‑Chat, Benutzer‑Aktivität |
| `has_dark_background` | `hasDarkBackground` | alle |
| `default_sort_direction` | `defaultSortDirection` | Kommentare, Live‑Chat, Kollaborativer/Bild‑Chat |
| `number_only` | `numberOnly` | Kommentar‑Zähler |
| `is_live` | `isLive` | Kommentar‑Zähler |
| `count` | `count` | aktuelle Kommentare, aktuelle Diskussionen |
| `target` | (querySelector, nicht gesendet) | Kollaborativer Chat, Bild‑Chat |
| `chat_square_percentage` | `chatSquarePercentage` | Bild‑Chat |
| `user_id` | `userId` | Benutzer‑Aktivität |

Beispiele:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Kommentare: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Kollaborativer Chat fügt sich an ein vorhandenes Element auf der Seite ein #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Bulk‑Zähler: Marker setzen, dann füllt ein Bulk‑Loader sie alle aus #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```