Cada widget tem sua própria tag. Todas aceitam argumentos de palavra-chave `**extra`,
que são mesclados na configuração do widget como estão (use chaves em camelCase) para qualquer
coisa não coberta pelos argumentos nomeados abaixo.

| Tag | Widget |
|---|---|
| `{% fastcomments %}` | Comentários |
| `{% fastcomments_live_chat %}` | Chat ao vivo |
| `{% fastcomments_comment_count %}` | Selo de contagem de comentários |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | Contagens de comentários em massa |
| `{% fastcomments_collab_chat target="#el" %}` | Chat colaborativo (inline) |
| `{% fastcomments_image_chat target="#el" %}` | Chat de anotação de imagem |
| `{% fastcomments_recent_comments %}` | Comentários recentes |
| `{% fastcomments_recent_discussions %}` | Discussões recentes |
| `{% fastcomments_reviews_summary %}` | Resumo de avaliações |
| `{% fastcomments_top_pages %}` | Páginas mais comentadas |
| `{% fastcomments_user_activity user_id="..." %}` | Feed de atividade de um usuário |

Os argumentos nomeados mapeiam para as chaves de configuração camelCase do widget:

| Argumento | Chave de configuração | Tags |
|---|---|---|
| `url_id` | `urlId` | comentários, chat ao vivo, contagem de comentários, chat colaborativo/de imagem, comentários recentes, resumo de avaliações |
| `url` | `url` | comentários, chat ao vivo, chat colaborativo/de imagem |
| `readonly` | `readonly` | comentários, chat ao vivo, chat colaborativo/de imagem |
| `locale` | `locale` | comentários, chat ao vivo, chat colaborativo/de imagem, atividade do usuário |
| `has_dark_background` | `hasDarkBackground` | todos |
| `default_sort_direction` | `defaultSortDirection` | comentários, chat ao vivo, chat colaborativo/de imagem |
| `number_only` | `numberOnly` | contagem de comentários |
| `is_live` | `isLive` | contagem de comentários |
| `count` | `count` | comentários recentes, discussões recentes |
| `target` | (querySelector, não enviado) | chat colaborativo, chat de imagem |
| `chat_square_percentage` | `chatSquarePercentage` | chat de imagem |
| `user_id` | `userId` | atividade do usuário |

Exemplos:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# O chat colaborativo se anexa a um elemento existente na página #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Contagens em massa: coloque os marcadores, então um carregador em massa preenche todos #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```