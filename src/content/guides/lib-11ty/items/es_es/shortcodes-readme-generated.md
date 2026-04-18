| Shortcode | Description |
| --- | --- |
| `fastcomments` | Widget de comentarios con respuestas, votaciones y más |
| `fastcommentsCommentCount` | Muestra el recuento de comentarios de una página |
| `fastcommentsImageChat` | Comentarios con anotaciones en imágenes |
| `fastcommentsLiveChat` | Widget de chat en vivo |
| `fastcommentsCollabChat` | Comentarios colaborativos en línea |
| `fastcommentsRecentComments` | Comentarios recientes en todo el sitio |
| `fastcommentsRecentDiscussions` | Hilos de discusión activos recientemente |
| `fastcommentsReviewsSummary` | Resumen de reseñas con calificación por estrellas |
| `fastcommentsTopPages` | Páginas más discutidas |
| `fastcommentsUserActivityFeed` | Feed de actividad del usuario |

### Ejemplos

```njk
{# Recuento de comentarios en línea con el texto #}
This page has {% fastcommentsCommentCount { tenantId: "demo" } %} comments.

{# Chat en vivo #}
{% fastcommentsLiveChat { tenantId: "demo" } %}

{# Chat colaborativo — apuntar a un elemento de contenido mediante selector CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcommentsCollabChat { tenantId: "demo", target: "#post-body" } %}

{# Chat de imagen — apuntar a un elemento de imagen mediante selector CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image" />
{% fastcommentsImageChat { tenantId: "demo", target: "#hero" } %}

{# Resumen de reseñas #}
{% fastcommentsReviewsSummary { tenantId: "demo" } %}

{# Feed de actividad del usuario #}
{% fastcommentsUserActivityFeed { tenantId: "demo", userId: "demo:demo-user" } %}
```