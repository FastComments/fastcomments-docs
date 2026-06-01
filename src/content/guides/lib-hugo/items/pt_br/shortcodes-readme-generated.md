| Shortcode | Descrição |
| --- | --- |
| `fastcomments` | Comentários encadeados com respostas, votação e menções @ |
| `fastcomments-comment-count` | Contagem de comentários para uma única página |
| `fastcomments-comment-count-bulk` | Contagens de comentários para várias páginas em uma única solicitação (veja [Contagens de comentários em massa](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widget de chat ao vivo |
| `fastcomments-collab-chat` | Comentários colaborativos em linha (requer `target`) |
| `fastcomments-image-chat` | Comentários de anotação de imagem (requer `target`) |
| `fastcomments-recent-comments` | Comentários recentes em todo o site |
| `fastcomments-recent-discussions` | Tópicos de discussão recentemente ativos |
| `fastcomments-reviews-summary` | Resumo de avaliações com estrelas |
| `fastcomments-top-pages` | Páginas mais discutidas |
| `fastcomments-user-activity-feed` | Feed de atividade por usuário (requer `userId`) |

### Exemplos

Contagem de comentários embutida no texto:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Chat ao vivo:

```text
\{{< fastcomments-live-chat >}}
```

Chat colaborativo, direcionando um elemento de conteúdo por seletor CSS:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Chat de imagem, direcionando um elemento de imagem por seletor CSS:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Resumo de avaliações:

```text
\{{< fastcomments-reviews-summary >}}
```

Feed de atividade do usuário:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```