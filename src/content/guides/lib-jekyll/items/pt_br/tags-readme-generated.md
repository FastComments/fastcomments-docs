| Tag | Descrição |
| --- | --- |
| `fastcomments` | Comentários ao vivo com respostas, votação, moderação e atualizações em tempo real |
| `fastcomments_comment_count` | Contador de comentários para a página atual |
| `fastcomments_comment_count_bulk` | Contadores de comentários para várias páginas em uma página de lista/índice |
| `fastcomments_live_chat` | Widget de chat de streaming em tempo real |
| `fastcomments_collab_chat` | Comentário colaborativo inline (anotações de texto) |
| `fastcomments_image_chat` | Comentários de anotação de imagem |
| `fastcomments_recent_comments` | Comentários recentes em todo o site |
| `fastcomments_recent_discussions` | Tópicos de discussão recentemente ativos |
| `fastcomments_reviews_summary` | Resumo de avaliações com estrelas |
| `fastcomments_top_pages` | Páginas mais comentadas |
| `fastcomments_user_activity_feed` | Feed de atividade por usuário |

### Exemplos

```liquid
{% raw %}{# Contador de comentários. O widget gera seu próprio rótulo, ex.: "0 comentários" #}
{% fastcomments_comment_count %}

{# Chat ao vivo #}
{% fastcomments_live_chat %}

{# Chat colaborativo. Aponte para um elemento de conteúdo com um seletor CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Chat de imagem. Aponte para um elemento de imagem com um seletor CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Resumo de avaliações #}
{% fastcomments_reviews_summary %}

{# Feed de atividade do usuário. Requer um id de usuário #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Contadores de comentários em massa para um índice de blog #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```