Każdy widget ma swój własny tag. Wszystkie przyjmują argumenty słownikowe `**extra`,
które są scalane z konfiguracją widgetu w niezmienionej postaci (używaj kluczy w camelCase) dla wszystkiego,
czego nie obejmują nazwane argumenty poniżej.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Komentarze |
| `{% fastcomments_live_chat %}` | Czat na żywo |
| `{% fastcomments_comment_count %}` | Licznik komentarzy |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Zbiorcze liczniki komentarzy |
| `{% fastcomments_collab_chat target="#el" %}` | Czat współpracy (inline) |
| `{% fastcomments_image_chat target="#el" %}` | Czat z adnotacjami na obrazie |
| `{% fastcomments_recent_comments %}` | Ostatnie komentarze |
| `{% fastcomments_recent_discussions %}` | Ostatnie dyskusje |
| `{% fastcomments_reviews_summary %}` | Podsumowanie recenzji |
| `{% fastcomments_top_pages %}` | Najczęściej komentowane strony |
| `{% fastcomments_user_activity user_id="..." %}` | Aktywność użytkownika |

Nazwane argumenty mapują się na klucze konfiguracji widgetu w camelCase:

| Argument | Klucz konfiguracji | Tagi |
|---|---|---|
| `url_id` | `urlId` | komentarze, czat na żywo, licznik komentarzy, czat współpracy/obrazu, ostatnie komentarze, podsumowanie recenzji |
| `url` | `url` | komentarze, czat na żywo, czat współpracy/obrazu |
| `readonly` | `readonly` | komentarze, czat na żywo, czat współpracy/obrazu |
| `locale` | `locale` | komentarze, czat na żywo, czat współpracy/obrazu, aktywność użytkownika |
| `has_dark_background` | `hasDarkBackground` | wszystkie |
| `default_sort_direction` | `defaultSortDirection` | komentarze, czat na żywo, czat współpracy/obrazu |
| `number_only` | `numberOnly` | licznik komentarzy |
| `is_live` | `isLive` | licznik komentarzy |
| `count` | `count` | ostatnie komentarze, ostatnie dyskusje |
| `target` | (querySelector, nie wysyłane) | czat współpracy, czat obrazu |
| `chat_square_percentage` | `chatSquarePercentage` | czat obrazu |
| `user_id` | `userId` | aktywność użytkownika |

Przykłady:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Czat współpracy dołącza do istniejącego elementu na stronie #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Zbiorcze liczniki: umieść znaczniki, a jeden zbiorczy loader wypełni je wszystkie #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```