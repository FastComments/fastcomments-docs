## Disparadores

Los disparadores inician un Zap cuando ocurre algo en FastComments. Los tres son instantáneos: FastComments entrega el
evento a Zapier a través de un webhook en el momento en que ocurre. No se realiza ninguna encuesta a su cuenta y no se gastan créditos de API
esperando.

| Disparador | Se activa cuando |
|-----------|-------------------|
| Nuevo comentario | Se publica un comentario. Por defecto solo se activan los comentarios aprobados y que no son spam. |
| Comentario actualizado | Un comentario es editado, aprobado, votado, fijado, bloqueado o modificado de alguna otra forma. |
| Comentario eliminado | Se elimina un comentario. |

Cada disparador devuelve el comentario completo: id, URL de la página e ID de la URL, nombre y correo electrónico del comentarista, el texto del comentario como markdown y como HTML, recuentos de votos, indicadores de aprobación y spam, la configuración regional, el dominio y cualquier mención. Los campos coinciden con la carga útil del webhook documentada en Webhooks, Estructuras de datos.

## Opciones

**Dominio.** Cada disparador tiene un filtro de dominio opcional, que enumera los dominios configurados en su cuenta. Déjelo vacío para recibir eventos de todos los dominios.

**Incluir comentarios no aprobados y spam.** Solo en el disparador Nuevo comentario. Los comentarios que están en espera de moderación o marcados como spam se omiten por defecto. Cuando dicho comentario se aprueba más tarde, el disparador Comentario actualizado se activa para él, de modo que un Zap que debe reaccionar a cada comentario que se vuelve visible utiliza Comentario actualizado con un filtro en el campo aprobado.

## Cómo funciona la entrega

Activar un Zap crea una suscripción webhook en su cuenta, visible en la página Webhooks con la fuente **API**. Desactivar el Zap la elimina. Los propios límites de Zapier se aplican a cuántos eventos acepta por minuto; FastComments reintenta una entrega que falla, con un retraso creciente, y desactiva una suscripción que sigue fallando durante seis días. Una suscripción desactivada puede volver a habilitarse desde la página Webhooks, o simplemente desactive y active nuevamente el Zap para crear una nueva.

Una cuenta puede contener hasta 50 suscripciones API. Cada Zap que utiliza un disparador de FastComments usa una.