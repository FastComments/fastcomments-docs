---
Se activa cuando un moderador otorga una insignia a un usuario.

### Contexto que recibe el agente

- El **badge ID** de la insignia otorgada.
- El **triggering user ID** - el moderador que otorgó la insignia.
- Contexto opcional de hilo / historial del usuario / página según lo configurado.

El sitio de activación no incluye un `commentId` en la carga útil del evento, incluso si la insignia se otorgó con respecto a un comentario específico.

### Quién lo activa

Una acción de un moderador humano.

### Observaciones

- Solo se incluye el **badge ID**; el agente no recibe los metadatos de la insignia (nombre, imagen). Si el agente necesita razonar sobre *qué* insignia fue otorgada, incluya ese contexto en el [mensaje inicial](#personality-prompt) o en las [directrices de la comunidad](#community-guidelines).
- El disparador se activa una vez por otorgamiento de insignia, no por usuario. Otorgar la misma insignia a un usuario dos veces la dispara dos veces (cada otorgamiento es un evento distinto).

### Usos comunes

- **Reconocimiento recíproco** - un agente puede publicar una respuesta de "gracias por la gran contribución" cuando se otorga una insignia específica.
- **Flujo de reconocimiento externo** vía [Webhooks](#webhooks-overview) - reflejar los otorgamientos de insignias en su propio sistema de compromiso de usuarios.
- **Registro en memoria** - notas como "este usuario es un colaborador reconocido" para que los futuros agentes de moderación lo tengan en cuenta en sus decisiones.

---