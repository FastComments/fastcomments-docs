Se activa cuando un usuario publica su primer comentario en este sitio (su tenant). Esto es **una sola vez por usuario**: los comentarios posteriores del mismo usuario no lo vuelven a activar.

### Contexto que recibe el agente

- El nuevo comentario.
- Contexto opcional de hilo / historial del usuario / página según esté configurado.

Cuando el contexto de historial de usuario está activado, la lista de comentarios recientes del usuario, por supuesto, estará vacía (o contendrá solo este comentario), pero el factor de confianza y la edad de la cuenta sí se rellenan.

### Notas importantes

- "Primer comentario en este sitio" está limitado al **tenant**, no a todo el sitio a nivel de FastComments. Un usuario con comentarios en otros sitios de FastComments todavía activa este disparador la primera vez que publica en el suyo.
- El disparador solo se activa para usuarios que tienen un userId. Los comentarios anónimos no verificados sin un userId estable no lo activan.
- El disparador se activa cuando el comentario es aprobado/visible (no en el momento de la publicación inicial). Las ediciones o acciones de moderación en los primeros comentarios no lo vuelven a activar.

### Usos comunes

- **Mensaje de bienvenida** - la [plantilla Welcome Greeter](#template-welcome-greeter) está diseñada en torno a este disparador.
- **Incorporación** - enviar un [DM warning](#tool-warn-user) (usado aquí como un aviso amistoso más que como una advertencia) que señale al usuario las pautas de la comunidad.
- **Notificación al revisor** - si quieres que un humano revise la primera publicación de cada nuevo comentarista, [`mark_comment_reviewed`](#tools-overview) puede marcarlos para revisión.

---