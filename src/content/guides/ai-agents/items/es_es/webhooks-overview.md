---
Los webhooks de agente son callbacks HTTP disparados por la plataforma cuando se completa una ejecución de agente o cuando una aprobación cambia de estado. Úsalos para reenviar la actividad del agente a tus propios sistemas: paneles de moderación, registros de auditoría, canales de Slack y herramientas de escalado.

Configurado en la pestaña **Webhooks** de la [página de Agentes de IA](https://fastcomments.com/auth/my-account/ai-agents).

### What gets delivered

Four event types:

- **`trigger.succeeded`** - an agent run completed successfully.
- **`trigger.failed`** - an agent run errored.
- **`approval.requested`** - an action was queued for human approval.
- **`approval.decided`** - an approval was approved, rejected, or execution-failed.

Consulte [Eventos de webhook](#webhook-events) para saber qué eventos se desencadenan y cuándo, y [Cargas útiles de webhook](#webhook-payloads) para el esquema de cada uno.

### What's not delivered

- **Per-tool-action webhooks.** Una ejecución que llame a `pin_comment` no dispara un webhook separado para el pin: la acción se incluye en la carga útil `trigger.succeeded` de la ejecución. Si quieres entrega por acción, analiza el array `actions` en la carga útil del trigger.
- **Dropped triggers.** Un disparador que no se envía (por sobrepaso de presupuesto, ámbito incorrecto) no dispara un webhook. Los descartes son visibles solo en la [página de Analytics](#analytics-page).
- **Replay-produced triggers.** Las ejecuciones de prueba no generan webhooks. Consulte [Test Runs (Replays)](#test-runs-replays).

### Configuration

Para cada webhook que configures:

- **URL** - el endpoint HTTPS al que se hará POST.
- **Domain** - el dominio de comentarios al que se aplica este webhook (tu tenant puede alojar múltiples dominios). `*` matches all domains; `*.example.com` is a glob; an exact domain matches only that one.
- **Events** - a cuáles de los cuatro tipos de eventos suscribirse.
- **Agents** - vacío para "all agents", o una lista de IDs de agentes específicos para filtrar.
- **Method** - POST o PUT (por defecto POST).
- **No-retry status codes** - códigos de respuesta HTTP que deben considerarse fallos terminales y no reintentarse (por ejemplo, 410 Gone, 422 Unprocessable). Consulte [Webhook Retries](#webhook-retries).

Varios webhooks pueden suscribirse al mismo evento: cada uno se encola de forma independiente y se entrega a su propia URL.

### Per-domain matching

Un evento dado se entrega a **cada webhook cuyo campo `domain` coincida con el dominio del comentario**. La coincidencia usa un patrón glob simple:

- Exacto: `example.com` coincide solo con `example.com`.
- Asterisco comodín: `*` coincide con todos los dominios.
- Glob de subdominio: `*.example.com` coincide con `blog.example.com`, `forum.example.com`, pero no con `example.com` en sí.

El dominio en una carga útil es el primer resultado no nulo de: el `domain` del comentario, la URL analizada según la configuración de dominios de tu tenant, o la búsqueda de `Page` por `urlId`.

### Per-agent filtering

El campo **Agents** permite que un webhook se suscriba solo a ciertos agentes. Vacío significa "all agents". Cuando no está vacío, el webhook solo se activa para los agentes de la lista.

Esto es útil cuando tienes un webhook para eventos de moderación y otro para eventos de interacción, ambos dirigiéndose a sistemas downstream diferentes.

### Test send

La interfaz de configuración de webhooks tiene un botón **Enviar prueba** que envía una carga útil falsa a la URL para que puedas verificar la conectividad, la firma y el código de respuesta de tu endpoint sin esperar a un evento real.

### Delivery logs

Cada entrega (y cada reintento) aparece en la página de [Webhook Delivery Logs](#webhook-logs) para que puedas ver lo que ocurrió en la transmisión.

### Signing

Cada webhook se firma con HMAC-SHA256 usando el secreto de API de tu tenant. Consulte [Webhook Signing](#webhook-signing).

### Eligibility

Los webhooks de agente requieren facturación válida en el tenant. Usan la misma infraestructura de firma/secretos que tus webhooks de comentarios existentes: si ya has integrado los webhooks de comentarios (véase la [guía de Webhooks](/guide-webhooks.html)), la integración de webhooks de agente tiene la misma forma con nuevos tipos de eventos.

---