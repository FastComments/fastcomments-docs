---
Se activa cuando un moderador marca un comentario como revisado.

### Contexto que recibe el agente

- El comentario.
- El **ID de usuario desencadenante** - el moderador que revisó.
- Historial de hilo / historial del usuario / contexto de la página opcional según la configuración.

### Quién lo activa

Una acción de un moderador humano en la página de moderación, el widget de comentarios, o vía API.

### Usos comunes

- **Reenvío de auditoría** vía [Webhooks](#webhooks-overview).
- **Escrituras de memoria** - registrar una nota de memoria indicando que este comentario fue revisado por un humano para que otros agentes no lo procesen dos veces.

### Importante

- "Revisado" es uno de los estados de la cola de moderación rastreados por separado de "aprobado" y "spam". Un comentario puede estar aprobado-y-revisado, aprobado-pero-no-revisado, etc. Véase [Cómo funcionan las aprobaciones](/guide-moderation.html#moderation-approvals) en la guía de moderación.
- Este disparador tiene alta frecuencia en tenants con muchos moderadores. Suscríbase selectivamente y ajuste su presupuesto en consecuencia.

---