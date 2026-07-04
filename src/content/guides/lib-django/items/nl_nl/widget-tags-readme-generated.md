Elke widget heeft zijn eigen tag. Alle widgets accepteren `**extra` keyword‑argumenten,
die ongewijzigd worden samengevoegd in de widgetconfiguratie (gebruik camelCase‑sleutels) voor alles
dat niet wordt gedekt door de hieronder genoemde argumenten.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Reacties |
| `{% fastcomments_live_chat %}` | Livechat |
| `{% fastcomments_comment_count %}` | Reactieteller badge |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Bulk reactietellingen |
| `{% fastcomments_collab_chat target="#el" %}` | Samenwerkende (inline) chat |
| `{% fastcomments_image_chat target="#el" %}` | Afbeeldingsannotatie chat |
| `{% fastcomments_recent_comments %}` | Recente reacties |
| `{% fastcomments_recent_discussions %}` | Recente discussies |
| `{% fastcomments_reviews_summary %}` | Samenvatting van recensies |
| `{% fastcomments_top_pages %}` | Meest besproken pagina's |
| `{% fastcomments_user_activity user_id="..." %}` | Activiteitsfeed van een gebruiker |

Naamgegeven argumenten worden gemapt naar de camelCase‑configuratiesleutels van de widget:

| Argument | Config key | Tags |
|---|---|---|
| `url_id` | `urlId` | reacties, livechat, reactieteller, samenwerkende (inline) chat/afbeeldingsannotatie chat, recente reacties, samenvatting van recensies |
| `url` | `url` | reacties, livechat, samenwerkende (inline) chat/afbeeldingsannotatie chat |
| `readonly` | `readonly` | reacties, livechat, samenwerkende (inline) chat/afbeeldingsannotatie chat |
| `locale` | `locale` | reacties, livechat, samenwerkende (inline) chat/afbeeldingsannotatie chat, gebruikersactiviteit |
| `has_dark_background` | `hasDarkBackground` | alle |
| `default_sort_direction` | `defaultSortDirection` | reacties, livechat, samenwerkende (inline) chat/afbeeldingsannotatie chat |
| `number_only` | `numberOnly` | reactieteller |
| `is_live` | `isLive` | reactieteller |
| `count` | `count` | recente reacties, recente discussies |
| `target` | (querySelector, not sent) | samenwerkende (inline) chat, afbeeldings chat |
| `chat_square_percentage` | `chatSquarePercentage` | afbeeldings chat |
| `user_id` | `userId` | gebruikersactiviteit |

Voorbeelden:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Samenwerkende chat wordt gekoppeld aan een bestaand element op de pagina #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Bulk tellers: plaats markers, daarna vult één bulkloader ze allemaal in #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```