| Shortcode | Descripción |
| --- | --- |
| `fastcomments` | Comentarios en hilos con respuestas, votación y menciones @ |
| `fastcomments-comment-count` | Recuento de comentarios para una sola página |
| `fastcomments-comment-count-bulk` | Recuentos de comentarios para muchas páginas en una sola solicitud (ver [Recuentos de comentarios en bloque](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | Widget de chat en vivo |
| `fastcomments-collab-chat` | Comentarios colaborativos en línea (requiere `target`) |
| `fastcomments-image-chat` | Comentarios de anotación de imagen (requiere `target`) |
| `fastcomments-recent-comments` | Comentarios recientes en todo el sitio |
| `fastcomments-recent-discussions` | Hilos de discusión recientemente activos |
| `fastcomments-reviews-summary` | Resumen de reseñas con clasificación por estrellas |
| `fastcomments-top-pages` | Páginas más comentadas |
| `fastcomments-user-activity-feed` | Feed de actividad por usuario (requiere `userId`) |

### Ejemplos

Recuento de comentarios en línea con el texto:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

Chat en vivo:

```text
\{{< fastcomments-live-chat >}}
```

Chat colaborativo, apuntando a un elemento de contenido mediante un selector CSS:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

Chat de imagen, dirigido a un elemento de imagen mediante un selector CSS:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

Resumen de reseñas:

```text
\{{< fastcomments-reviews-summary >}}
```

Feed de actividad del usuario:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```