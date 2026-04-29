Activa el agente cada vez que se publica un nuevo comentario en una página cubierta por el [ámbito](#scope-url-locale) del agente.

### Contexto que recibe el agente

- El nuevo comentario en su totalidad: texto, autor, votos, ID del padre, ID de la URL de la página.
- Opcional: el comentario padre y respuestas previas en el mismo hilo, si el [contexto de hilo](#context-options) está activado.
- Opcional: el factor de confianza del comentarista, antigüedad de la cuenta, historial de suspensiones y comentarios recientes, si el [contexto del historial de usuario](#context-options) está activado.
- Opcional: metadatos de la página, si el [contexto de la página](#context-options) está activado.

### Destacado

- El trigger se activa **después** de que el comentario se haya persistido. El agente puede referirse a él directamente en las llamadas a herramientas.
- No se activa para comentarios redactados por otro agente en el mismo tenant.
- Se activa tanto para comentarios verificados como no verificados. Si su tenant requiere la aprobación de un moderador antes de que un comentario sea visible (véase [Cómo funcionan las aprobaciones](/guide-moderation.html#moderation-approvals) en la guía de moderación), el trigger se activa cuando se crea el comentario, no cuando se aprueba más tarde. Se puede instruir al bot moderador para que apruebe los comentarios por usted tras la revisión.

### Usos comunes

- **Moderación** - verificar el comentario frente a las directrices de la comunidad, marcar spam o advertir a los usuarios primerizos.
- **Saludo de bienvenida** - aunque [Trigger: Primer comentario de un nuevo usuario](#trigger-new-user-first-comment) suele ser una mejor opción para saludos, ya que se activa una vez por usuario.
- **Resumen de hilo** - normalmente se combina con un [retraso del trigger](#trigger-deferred-delay) para que el hilo se asiente antes de que se ejecute el agente.