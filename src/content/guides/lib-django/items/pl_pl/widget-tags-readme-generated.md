Every widget has its own tag. All of them accept `**extra` keyword arguments,
which are merged into the widget config as‑is (use camelCase keys) for anything
not covered by the named arguments below.

| Znacznik | Widżet |
|---|---|
| `{% fastcomments %}` | Komentarze |
| `{% fastcomments_live_chat %}` | Czat na żywo |
| `{% fastcomments_comment_count %}` | Odznaka liczby komentarzy |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Zbiorcze liczniki komentarzy |
| `{% fastcomments_collab_chat target="#el" %}` | Współpracujący (wbudowany) czat |
| `{% fastcomments_image_chat target="#el" %}` | Czat adnotacji obrazów |
| `{% fastcomments_recent_comments %}` | Ostatnie komentarze |
| `{% fastcomments_recent_discussions %}` | Ostatnie dyskusje |
| `{% fastcomments_reviews_summary %}` | Podsumowanie recenzji |
| `{% fastcomments_top_pages %}` | Najbardziej dyskutowane strony |
| `{% fastcomments_user_activity user_id="..." %}` | Kanał aktywności użytkownika |

Named arguments map to the widget's camelCase config keys:

| Argument | Config key | Znaczniki |
|---|---|---|
| `url_id` | `urlId` | komentarze, czat na żywo, licznik komentarzy, czat współpracy/obrazkowy, ostatnie komentarze, podsumowanie recenzji |
| `url` | `url` | komentarze, czat na żywo, czat współpracy/obrazkowy |
| `readonly` | `readonly` | komentarze, czat na żywo, czat współpracy/obrazkowy |
| `locale` | `locale` | komentarze, czat na żywo, czat współpracy/obrazkowy, aktywność użytkownika |
| `has_dark_background` | `hasDarkBackground` | wszystkie |
| `default_sort_direction` | `defaultSortDirection` | komentarze, czat na żywo, czat współpracy/obrazkowy |
| `number_only` | `numberOnly` | licznik komentarzy |
| `is_live` | `isLive` | licznik komentarzy |
| `count` | `count` | ostatnie komentarze, ostatnie dyskusje |
| `target` | (querySelector, not sent) | czat współpracy, czat adnotacji obrazów |
| `chat_square_percentage` | `chatSquarePercentage` | czat adnotacji obrazów |
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

{# Zliczenia zbiorcze: umieść znaczniki, a następnie jeden loader zbiorczy wypełni je wszystkie #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```