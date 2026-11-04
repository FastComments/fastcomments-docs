Los webhooks de agentes son callbacks HTTP que la plataforma dispara cuando una ejecución de agente se completa o una aprobación cambia de estado. Úsalos para reenviar la actividad de los agentes a tus propios sistemas: paneles de moderación, registros de auditoría, canales de Slack, herramientas de escalado.

Se configuran en la pestaña **Webhooks** de la [página de AI Agents](https://fastcomments.com/auth/my-account/ai-agents).

### Qué se entrega

Cuatro tipos de eventos:

- **`trigger.succeeded`** - una ejecución de agente se completó con éxito.
- **`trigger.failed`** - una ejecución de agente falló con error.
- **`approval.requested`** - una acción fue puesta en cola para aprobación humana.
- **`approval.decided`** - una aprobación fue aprobada, rechazada o falló en su ejecución.

Consulta [Webhook Events](#webhook-events) para saber qué eventos se disparan y cuándo, y [Webhook Payloads](#webhook-payloads) para el esquema de cada uno.

### Qué no se entrega

- **Webhooks por acción de herramienta.** Una ejecución que llama a `pin_comment` no dispara un webhook separado para el pin; la acción se incluye en el payload `trigger.succeeded` de la ejecución. Si quieres entrega por acción, analiza el array `actions` del payload del trigger.
- **Triggers descartados.** Un trigger que no se despacha (por encima del presupuesto, ámbito incorrecto) no dispara un webhook. Los descartes solo son visibles en la [página de Analytics](#analytics-page).
- **Triggers producidos por replay.** Las ejecuciones de prueba no disparan webhooks. Consulta [Test Runs (Replays)](#test-runs-replays).

### Configuración

Para cada webhook configuras:

- **URL** - el endpoint HTTPS al que se hace POST.
- **Domain** - el dominio de comentarios al que aplica este webhook (tu tenant puede alojar varios dominios). `*` coincide con todos los dominios; `*.example.com` es un glob; un dominio exacto coincide solo con ese.
- **Events** - a cuáles de los cuatro tipos de eventos suscribirse.
- **Agents** - vacío para "todos los agentes", o una lista de IDs de agentes específicos para filtrar.
- **Method** - POST o PUT (por defecto POST).
- **No-retry status codes** - códigos de respuesta HTTP que deben tratarse como fallos terminales, sin reintentar (por ejemplo, 410 Gone, 422 Unprocessable). Consulta [Webhook Retries](#webhook-retries).

Varios webhooks pueden suscribirse al mismo evento; cada uno se pone en cola de forma independiente y se entrega a su propia URL.

### Coincidencia por dominio

Un evento determinado se entrega a **todos los webhooks cuyo campo `domain` coincida con el dominio del comentario**. La coincidencia usa un glob simple:

- Exacto: `example.com` coincide solo con `example.com`.
- Comodín estrella: `*` coincide con todos los dominios.
- Glob de subdominio: `*.example.com` coincide con `blog.example.com`, `forum.example.com`, pero no con `example.com` en sí.

El dominio en un payload es el primer resultado no nulo de: el `domain` del comentario, la URL analizada contra la configuración de dominios de tu tenant, o la búsqueda de `Page` por `urlId`.

### Filtrado por agente

El campo **Agents** permite que un webhook se suscriba solo a ciertos agentes. Vacío significa "todos los agentes". Cuando no está vacío, el webhook solo se dispara para los agentes de la lista.

Esto es útil cuando tienes un webhook para eventos de moderación y otro para eventos de interacción, ambos enrutados a diferentes sistemas posteriores.

### Envío de prueba

La interfaz de configuración de webhooks tiene un botón **Test send** que publica un payload falso a la URL para que puedas verificar la conectividad, la firma y el código de respuesta de tu endpoint sin esperar un evento real.

### Registros de entrega

Cada entrega (y cada reintento) queda registrada en la página [Webhook Delivery Logs](#webhook-logs) para que puedas ver qué ocurrió en la comunicación.

### Firma

Cada webhook se firma con HMAC-SHA256 usando el secreto de API de tu tenant. Consulta [Webhook Signing](#webhook-signing).

### Elegibilidad

Los webhooks de agentes requieren una facturación válida en el tenant. Usan la misma infraestructura de firma/secreto que tus webhooks de comentarios existentes; si ya has integrado webhooks de comentarios (consulta la [guía de Webhooks](/guide-webhooks.html)), la integración de webhooks de agentes tiene la misma forma con nuevos tipos de eventos.