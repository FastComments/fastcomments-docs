Chaque widget possède sa propre balise. Toutes acceptent les arguments mot‑clé `**extra`, qui sont fusionnés dans la configuration du widget tels quels (utilisez des clés en camelCase) pour tout ce qui n’est pas couvert par les arguments nommés ci‑dessous.

| Balise | Widget |
|---|---|
| `{% fastcomments %}` | Commentaires |
| `{% fastcomments_live_chat %}` | Chat en direct |
| `{% fastcomments_comment_count %}` | Badge de nombre de commentaires |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Comptes de commentaires en masse |
| `{% fastcomments_collab_chat target="#el" %}` | Chat collaboratif (en ligne) |
| `{% fastcomments_image_chat target="#el" %}` | Chat d’annotation d’image |
| `{% fastcomments_recent_comments %}` | Commentaires récents |
| `{% fastcomments_recent_discussions %}` | Discussions récentes |
| `{% fastcomments_reviews_summary %}` | Résumé des avis |
| `{% fastcomments_top_pages %}` | Pages les plus commentées |
| `{% fastcomments_user_activity user_id="..." %}` | Flux d'activité d'un utilisateur |

Les arguments nommés correspondent aux clés de configuration camelCase du widget :

| Argument | Clé de configuration | Balises |
|---|---|---|
| `url_id` | `urlId` | commentaires, chat en direct, compteur de commentaires, chat collaboratif/image, commentaires récents, résumé des avis |
| `url` | `url` | commentaires, chat en direct, chat collaboratif/image |
| `readonly` | `readonly` | commentaires, chat en direct, chat collaboratif/image |
| `locale` | `locale` | commentaires, chat en direct, chat collaboratif/image, activité utilisateur |
| `has_dark_background` | `hasDarkBackground` | toutes |
| `default_sort_direction` | `defaultSortDirection` | commentaires, chat en direct, chat collaboratif/image |
| `number_only` | `numberOnly` | compteur de commentaires |
| `is_live` | `isLive` | compteur de commentaires |
| `count` | `count` | commentaires récents, discussions récentes |
| `target` | (querySelector, not sent) | chat collaboratif, chat image |
| `chat_square_percentage` | `chatSquarePercentage` | chat image |
| `user_id` | `userId` | activité utilisateur |

Exemples :

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Le chat collaboratif se branche à un élément existant sur la page #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Comptes en masse : placez des marqueurs, puis un chargeur en masse les remplit tous #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```
---