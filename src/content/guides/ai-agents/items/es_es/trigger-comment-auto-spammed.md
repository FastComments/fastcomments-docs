Se activa cuando un comentario es marcado automáticamente como spam por el motor de spam integrado de FastComments - **no** por un moderador y tampoco por otro agente.

### Contexto que recibe el agente

- El comentario marcado automáticamente como spam.
- Historial opcional del hilo / del usuario / contexto de la página según esté configurado.

### Quién lo dispara

- La canalización de spam de la plataforma. Véase [Detección de spam](/guide-moderation.html#spam-detection) en la guía de moderación para más detalles.

### Usos comunes

- **Moderación de segunda revisión** - el motor antispam tiene alta exhaustividad (recall) pero precisión imperfecta; un agente entrenado en el estilo específico de tu comunidad puede detectar falsos positivos. El agente puede llamar para desmarcar un comentario clasificado erróneamente.
- **Desbloqueo automático** - si tu tenant bloquea/agresivamente banea cuentas nuevas por spam, un agente activado por este disparador puede revisar y eliminar falsos positivos evidentes antes de que un humano los vea.

### Observaciones

- El disparador **no** se activa para el spam marcado por un moderador (usa [Disparador: Spam marcado por moderador](#trigger-moderator-spammed)) ni para el spam marcado por otro agente.
- Un comentario que es marcado automáticamente como spam y luego es marcado como No Spam por un moderador no vuelve a activar el disparador.
- Suscribirse a este disparador es más útil en tenants donde el modo de auto-spam del motor está habilitado en la Configuración de Moderación. De lo contrario, el disparador no se activará.

---