| Shortcode | Descrição |
| --- | --- |
| `fastcomments` | Widget de comentários com respostas, votação e mais |
| `fastcommentsCommentCount` | Exibe a contagem de comentários de uma página |
| `fastcommentsImageChat` | Comentários com anotação em imagens |
| `fastcommentsLiveChat` | Widget de chat ao vivo |
| `fastcommentsCollabChat` | Comentário colaborativo em linha |
| `fastcommentsRecentComments` | Comentários recentes no site |
| `fastcommentsRecentDiscussions` | Tópicos de discussão recentemente ativos |
| `fastcommentsReviewsSummary` | Resumo de avaliações com estrelas |
| `fastcommentsTopPages` | Páginas mais discutidas |
| `fastcommentsUserActivityFeed` | Feed de atividade do usuário |

### Exemplos

```njk
{# Contagem de comentários embutida no texto #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Chat ao vivo #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Chat colaborativo — apontar para um elemento de conteúdo usando seletor CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Chat de imagem — apontar para um elemento de imagem usando seletor CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Resumo de avaliações #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Feed de atividade do usuário #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```