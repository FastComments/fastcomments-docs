Cada webhook de agente tiene su propio registro de entregas. Accesible desde la [página de lista de webhooks](https://fastcomments.com/auth/my-account/ai-agents/webhooks) mediante el botón **Registros** en cada fila del webhook.

### Qué hay en la página

Una tabla paginada con una fila por intento de entrega:

| Columna | Significado |
|---|---|
| Cuándo | Cuándo ocurrió el intento. |
| Evento | El tipo de evento (`trigger.succeeded`, `approval.requested`, etc.). |
| Estado | El estado de la entrega. |
| StatusCode | Código de estado HTTP devuelto por tu endpoint, cuando está disponible. |
| Descripción | Una breve descripción del resultado. Para entregas fallidas donde no se recibió respuesta HTTP, el mensaje de error subyacente se almacena como `{error: <message>}`. |

La página solo admite paginación: no hay filtros por estado, tipo de evento o rango de fechas.

### Qué puedes hacer desde los registros

- **Decide si un código de estado debe incluirse en los códigos de estado sin reintento.** Si ves que tu endpoint devuelve repetidamente el mismo `4xx`, es una señal clara de que deberías añadirlo a **códigos de estado sin reintento** para que la plataforma deje de reintentar.

### Información sobre fallos

Cuando una entrega falla sin una respuesta HTTP (fallo de DNS, conexión rechazada, tiempo de espera, error TLS, etc.), el mensaje de error bruto se registra como `{error: <message>}`. La plataforma no categoriza estos en grupos con nombre como `TIMEOUT` o `DNS_ERROR`: lee el mensaje de error directamente para ver qué ocurrió.

Para respuestas HTTP, la columna StatusCode muestra el código devuelto por tu endpoint. Casos comunes:

- **Todos los intentos: `401` o `403`** - tu endpoint está rechazando la firma. Comprueba que estás calculando el HMAC sobre `${timestamp}.${body}` y usando el secreto correcto. Ver [Firma de webhooks](#webhook-signing).
- **Todos los intentos: `422`** - tu endpoint piensa que la carga útil es inválida. O bien corrige tu endpoint, o añade `422` a **códigos de estado sin reintento** si el rechazo se espera para algunos eventos.

### Contexto por entrega

Cada entrada de registro incluye:

- `webhookId` - qué configuración de webhook produjo esta entrega.
- `agentId` - sobre qué agente trata la entrega.
- `triggerId` or `approvalId` - el registro subyacente.
- `domain` - el dominio coincidente.

Puedes usar estos datos para correlacionar una entrega fallida con la ejecución a la que se relaciona en [Historial de ejecuciones](#run-history).

### Retención

Las entradas de `AgentSyncLog` tienen un TTL fijo de 1 año sobre `createdAt` independientemente del resultado: las entregas exitosas y fallidas se retienen por el mismo periodo. Tras caducar la retención, la entrada del registro desaparece.

Si necesitas auditoría a largo plazo, la práctica sostenible es: que sea el **propio endpoint** el que persista las entregas que recibe. Eso desacopla tu registro de auditoría de la política de retención de la plataforma.

### Envío de prueba

El botón **Enviar prueba** del formulario de configuración del webhook escribe una entrega falsa en la misma tabla de registros para que puedas verificar la conectividad de extremo a extremo antes de depender de eventos reales. Las entregas de prueba están claramente etiquetadas en el registro para que no contaminen las estadísticas de fallos de producción.

### Véase también

- [Descripción general de webhooks](#webhooks-overview).
- [Reintentos de webhooks](#webhook-retries) para la semántica de reintentos que generan estos registros.
- [Firma de webhooks](#webhook-signing) para cómo verificar en tu endpoint.