Se dispara cuando un moderador aprueba un comentario.

### Contexto que recibe el agente

- El comentario recién aprobado.
- **ID de usuario que activó** - el moderador que aprobó.
- Historial opcional de hilo / usuario / contexto de página según esté configurado.

### Quién lo dispara

Una acción realizada por un moderador humano.

### Notable

- Un comentario "aprobado" es un comentario **visible** en la terminología de FastComments. Consulta [Cómo funcionan las aprobaciones](/guide-moderation.html#moderation-approvals) en la guía de moderación para la distinción entre aprobado/no aprobado y revisado/no revisado.
- El disparador se activa en las **transiciones** de aprobación: un comentario que pasa de no aprobado a aprobado lo activa; un comentario que ya estaba aprobado y se vuelve a guardar no lo hace.
- Para los tenants donde los comentarios se aprueban automáticamente por defecto, este disparador se activa solo cuando un moderador vuelve a aprobar explícitamente un comentario que estaba oculto anteriormente.

### Usos comunes

- **Bienvenida / participación** - un agente puede responder a los comentaristas primerizos en el momento en que un moderador los aprueba, en lugar de en el momento de la publicación.
- **Coordinación entre agentes** - si un agente distinto había marcado el comentario para revisión, la aprobación es la señal de que la revisión humana ha finalizado.
- **Registro de auditoría** vía [Webhooks](#webhooks-overview).

---