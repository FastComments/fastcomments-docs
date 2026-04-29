Los webhooks de agente reintentan en caso de fallo. La entrega es **fire-and-forget desde la perspectiva del agente**: una entrega fallida no bloquea la ejecución del agente ni revierte ninguna acción; además, una cola + cron impulsa los reintentos de forma asíncrona.

### Queue model

Cada evento se pone en cola **una vez por webhook coincidente**. Así que si tienes tres webhooks suscritos a `trigger.succeeded` para un agente + dominio determinados, la plataforma encola tres entregas; cada una se entrega y reintenta de forma independiente. Un fallo en un webhook nunca afecta a los demás.

### What's retried

Se reintenta una entrega cuando:

- La petición HTTP **no se completa** (falla DNS, conexión rechazada, tiempo de espera).
- El código de respuesta HTTP es cualquier estado no 2xx que no esté en la lista configurada de **No-retry status codes**.

No se reintenta una entrega cuando:

- El código de respuesta es `2xx` (éxito).
- El código de respuesta está en la lista configurada de **No-retry status codes**. Por defecto esta lista está vacía: cualquier no-2xx se reintenta.

### Configuring no-retry codes

El formulario de configuración del webhook tiene un campo **No-retry status codes** (valores múltiples). Entradas comunes:

- `410` - Gone. Tu endpoint se ha movido permanentemente o el recurso ha desaparecido. Reintentar solo desperdicia ancho de banda en ambos extremos.
- `422` - Unprocessable Entity. Tu endpoint entendió la carga pero la consideró inválida. Reintentar con la misma carga devolverá la misma respuesta.
- `400` - Bad Request, en el mismo sentido.

Añadir un código aquí significa: cuando el endpoint lo devuelva, marca la entrega como fallida-terminal y deja de reintentar.

### Retry schedule

Un trabajador en segundo plano se ejecuta cada pocos segundos y procesa cualquier entrega cuya hora del siguiente intento haya pasado.

Después de cada fallo, la hora del siguiente intento se aplaza con **linear backoff**: la espera crece como `60 seconds * attempt count` (así que el intento 1 espera 1 minuto, el intento 2 espera 2 minutos, y así sucesivamente).

Después de 99 intentos fallidos (o 3 en desarrollo local), se abandona la entrega y se elimina de la cola. Las entradas del registro de entrega sí persisten y permanecen visibles en la página [Registros de entrega de webhook](#webhook-logs) hasta que expiran.

### Idempotence on your side

Porque reintentamos, tu endpoint **debe ser idempotente**. El mismo `triggerId` (o `approvalId`) puede llegar más de una vez. Tu endpoint debería:

- Usar una clave única (`triggerId` para eventos de trigger, `approvalId` para eventos de approval) como token de deduplicación.
- Aceptar entregas duplicadas con gracia (devolver 200 la segunda vez).

Un endpoint no idempotente eventualmente procesará doble algunas entregas, especialmente durante cortes transitorios donde un timeout reintenta 30 segundos después pero la petición original realmente tuvo éxito.

### Ordering

Las entregas **no están estrictamente ordenadas**. Un `trigger.succeeded` y un posterior `approval.requested` (del mismo run) pueden llegar en cualquier orden si uno se reintenta y el otro no. Tu endpoint no debe asumir un orden causal.

Si necesitas orden, usa las marcas de tiempo: `occurredAt` en el sobre, además del `createdAt` del trigger/approval en el bloque de datos, para reconstruir el orden por tu parte.

### Cleanup

Las entregas se eliminan de la cola tan pronto como tienen éxito o alcanzan el límite de intentos. La plataforma no mantiene las entregas fallidas terminalmente en la propia cola; el registro durable de cada intento vive en la página de [Registros de entrega de webhook](#webhook-logs).

### Where to look when retries fail

La página de [Registros de entrega de webhook](#webhook-logs) es el lugar para ver por qué falla un webhook. Causas comunes:

- **DNS resolution failure** - la URL es incorrecta o el dominio ha desaparecido.
- **TLS errors** - el certificado de tu endpoint es inválido o ha expirado.
- **Connection refused / timeout** - tu endpoint está caído.
- **5xx responses** - tu endpoint está arriba pero tuvo un error. El cuerpo de la respuesta (truncado) queda registrado.
- **4xx responses** - tu endpoint rechazó la carga. Si esto es intencional, añade el código a **No-retry status codes**.

### Pause an unhealthy webhook

Si un webhook está fallando de forma consistente, la solución más limpia es eliminarlo (o borrar temporalmente su lista de suscripción de eventos). La plataforma no desactiva automáticamente los webhooks que fallan: seguirán reintentando hasta que la entrega se dé por agotada.