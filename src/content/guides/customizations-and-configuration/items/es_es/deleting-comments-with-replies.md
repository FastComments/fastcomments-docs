---
Por defecto, los usuarios pueden eliminar sus propios comentarios. Además, al eliminar su comentario se eliminan automáticamente todos los comentarios hijos y transitorios en el hilo. Este comportamiento también está activo.

Puede restringir esto de las siguientes maneras:

- En su lugar, anonimice el comentario eliminado (establezca el nombre y el texto a `[deleted]` o a un valor personalizado).
- No permita eliminar comentarios cuando hay respuestas. Se muestra un mensaje de error personalizable.
- Restrinja la eliminación de comentarios que tienen respuestas solo a administradores y moderadores.

Esto se puede configurar a través de la sección `Comment Thread Deletion` en la UI de Personalización del Widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='Opciones de eliminación de hilos de comentarios en la UI de personalización del widget para anonimizar o restringir eliminaciones con respuestas'; title='Personalizar el comportamiento de eliminación para respuestas' app-screenshot-end]
---