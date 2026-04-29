Se activa cuando un comentario es bloqueado.

### Contexto que recibe el agente

- El comentario bloqueado.
- Hilo opcional / historial del usuario / contexto de la página según esté configurado.

### Quién lo activa

- Un moderador que usa la acción de bloqueo en la página de moderación o en el widget de comentarios.

### Usos comunes

- **Notificar a los revisores** - un evento de bloqueo a menudo sigue a un hilo acalorado; un webhook hacia tu canal de moderación en Slack puede permitir que los humanos se encarguen del resto.
- **Aplicación del periodo de enfriamiento** - programa un [disparador diferido](#trigger-deferred-delay) en un agente separado que, 24 horas después del bloqueo, considere si desbloquear.

### Par

Ver [Disparador: Comentario desbloqueado](#trigger-comment-unlock) para el disparador espejo.

---