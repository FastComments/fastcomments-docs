| Etiqueta | Descripción |
| --- | --- |
| `fastcomments` | Comentarios en vivo con respuestas, votación, moderación y actualizaciones en tiempo real |
| `fastcomments_comment_count` | Recuento de comentarios para la página actual |
| `fastcomments_comment_count_bulk` | Recuentos de comentarios para muchas páginas en una página de lista/índice |
| `fastcomments_live_chat` | Widget de chat en tiempo real (streaming) |
| `fastcomments_collab_chat` | Comentarios colaborativos inline (anotaciones de texto) |
| `fastcomments_image_chat` | Comentarios de anotación en imágenes |
| `fastcomments_recent_comments` | Comentarios recientes en todo el sitio |
| `fastcomments_recent_discussions` | Hilos de discusión recientemente activos |
| `fastcomments_reviews_summary` | Resumen de reseñas con calificación por estrellas |
| `fastcomments_top_pages` | Páginas más comentadas |
| `fastcomments_user_activity_feed` | Feed de actividad por usuario |

### Ejemplos

```liquid
{% raw %}{# Recuento de comentarios. El widget muestra su propia etiqueta, p. ej. "0 comentarios" #}
{% fastcomments_comment_count %}

{# Chat en vivo #}
{% fastcomments_live_chat %}

{# Chat colaborativo. Apúntalo a un elemento de contenido con un selector CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Chat de imagen. Apúntalo a un elemento de imagen con un selector CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# Resumen de reseñas #}
{% fastcomments_reviews_summary %}

{# Feed de actividad de usuario. Requiere un ID de usuario #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# Recuentos masivos de comentarios para un índice de blog #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```