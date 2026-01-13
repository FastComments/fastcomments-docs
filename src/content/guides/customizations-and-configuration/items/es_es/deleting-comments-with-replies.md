---
Por defecto, los usuarios pueden eliminar sus propios comentarios. Además, al eliminar su comentario automáticamente
se eliminan todos los comentarios secundarios y transitorios en el hilo. Este comportamiento también está activo.

Puedes restringir esto de las siguientes maneras:

- En su lugar, anonimizar el comentario eliminado (establecer name y text a `[deleted]` o un valor personalizado).
- No permitir eliminar comentarios cuando hay respuestas. Se muestra un mensaje de error personalizable.
- Restringir la eliminación de comentarios con respuestas únicamente a administradores y moderadores.

Esto se puede configurar a través de la sección `Comment Thread Deletion` en la interfaz de personalización del widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---