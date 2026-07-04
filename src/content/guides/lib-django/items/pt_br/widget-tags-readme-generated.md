Every widget has its own tag. All of them accept `**extra` keyword arguments,
which are merged into the widget config as‑is (use camelCase keys) for anything
not covered by the named arguments below.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Comentários |
| `{% fastcomments_live_chat %}` | Chat ao vivo |
| `{% fastcomments_comment_count %}` | Badge de contagem de comentários |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Contagens de comentários em lote |
| `{% fastcomments_collab_chat target="#el" %}` | Chat colaborativo (inline) |
| `{% fastcomments_image_chat target="#el" %}` | Chat de anotação de imagem |
| `{% fastcomments_recent_comments %}` | Comentários recentes |
| `{% fastcomments_recent_discussions %}` | Discussões recentes |
| `{% fastcomments_reviews_summary %}` | Resumo de avaliações |
| `{% fastcomments_top_pages %}` | Páginas mais discutidas |
| `{% fastcomments_user_activity user_id="..." %}` | Feed de atividade de um usuário |

Named arguments map to the widget's camelCase config keys:

| Argument | Config key | Tags |
|---|---|---|
| `url_id` | `urlId` | comentários, chat ao vivo, contagem de comentários, chat colaborativo/imagem, comentários recentes, resumo de avaliações |
| `url` | `url` | comentários, chat ao vivo, chat colaborativo/imagem |
| `readonly` | `readonly` | comentários, chat ao vivo, chat colaborativo/imagem |
| `locale` | `locale` | comentários, chat ao vivo, chat colaborativo/imagem, atividade do usuário |
| `has_dark_background` | `hasDarkBackground` | todos |
| `default_sort_direction` | `defaultSortDirection` | comentários, chat ao vivo, chat colaborativo/imagem |
| `number_only` | `numberOnly` | contagem de comentários |
| `is_live` | `isLive` | contagem de comentários |
| `count` | `count` | comentários recentes, discussões recentes |
| `target` | (querySelector, not sent) | chat colaborativo, chat de imagem |
| `chat_square_percentage` | `chatSquarePercentage` | chat de imagem |
| `user_id` | `userId` | atividade do usuário |

Examples:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comentários: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Chat colaborativo se anexa a um elemento existente na página #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Contagens em lote: coloque os marcadores, então um carregador em lote os preenche todos #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```