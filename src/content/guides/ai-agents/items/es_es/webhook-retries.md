Los webhooks de agente reintentan en caso de fallo. La entrega es **fire-and-forget desde la perspectiva del agente** - una entrega fallida no bloquea la ejecución del agente ni revierte ninguna acción - y una cola + cron gestiona los reintentos de forma asíncrona.

### Modelo de cola

Cada evento se encola **una vez por webhook coincidente**. Así que si tienes tres webhooks suscritos a `trigger.succeeded` para un agente + dominio determinados, la plataforma encola tres entregas; cada una se entrega y se reintenta de forma independiente. Un fallo en un webhook nunca afecta a los demás.

### Qué se reintenta

Una entrega se reintenta cuando:

- La petición HTTP **no se completa** (fallo de DNS, conexión rechazada, timeout).
- El código de respuesta HTTP es cualquier estado no-2xx que no esté en la lista configurada de **No-retry status codes**.

Una entrega **no se reintenta** cuando:

- El código de respuesta es `2xx` (éxito).
- El código de respuesta está en la lista configurada de **No-retry status codes**. Por defecto esta lista está vacía - cualquier no-2xx se reintenta.

### Configurar códigos sin reintento

El formulario de configuración del webhook tiene un campo **No-retry status codes** (multi-valor). Entradas comunes:

- `410` - Gone. Tu endpoint se ha movido permanentemente o el recurso ya no existe. Reintentar solo desperdicia ancho de banda de ambas partes.
- `422` - Unprocessable Entity. Tu endpoint entendió el payload pero lo consideró inválido. Reintentar con el mismo payload dará la misma respuesta.
- `400` - Bad Request, en el mismo espíritu.

Añadir un código aquí significa: cuando el endpoint lo devuelva, marcar la entrega como fallida-terminal y dejar de reintentar.

### Calendario de reintentos

Un worker en segundo plano se ejecuta cada pocos segundos y procesa las entregas cuya hora de próximo intento ha pasado.

Tras cada fallo, la hora del próximo intento se pospone con **backoff lineal**: la espera crece como `60 segundos * número de intento` (así el intento 1 espera 1 minuto, el intento 2 espera 2 minutos, y así sucesivamente).

Tras 99 intentos fallidos (o 3 en desarrollo local), la entrega se abandona y se elimina de la cola. Las entradas del registro de entregas sí persisten y permanecen visibles en la página [Webhook Delivery Logs](#webhook-logs) hasta que expiran.

### Idempotencia en tu lado

Como reintentamos, tu endpoint **debe ser idempotente**. El mismo `triggerId` (o `approvalId`) puede llegar más de una vez. Tu endpoint debería:

- Usar una clave única (`triggerId` para eventos de trigger, `approvalId` para eventos de approval) como token de deduplicación.
- Aceptar entregas duplicadas con elegancia (devolver 200 la segunda vez).

Un endpoint no idempotente eventualmente procesará dos veces algunas entregas, especialmente durante interrupciones transitorias donde un timeout reintenta 30 segundos después pero la petición original realmente tuvo éxito.

### Ordenación

Las entregas **no están estrictamente ordenadas**. Un `trigger.succeeded` y un `approval.requested` posterior (de la misma ejecución) pueden llegar en cualquier orden si uno reintenta y el otro no. Tu endpoint no debe asumir un orden causal.

Si necesitas ordenación, usa las marcas de tiempo - `occurredAt` en el envelope, más el `createdAt` del trigger/approval en el bloque de datos - para reconstruir el orden en tu lado.

### Limpieza

Las entregas se eliminan de la cola tan pronto como tienen éxito o alcanzan el límite de intentos. La plataforma no retiene las entregas fallidas-terminalmente en la cola misma; el registro duradero de cada intento vive en la página [Webhook Delivery Logs](#webhook-logs).

### Dónde mirar cuando los reintentos fallan

La página [Webhook Delivery Logs](#webhook-logs) es el lugar para ver por qué falla un webhook. Causas comunes:

- **Fallo de resolución DNS** - la URL es incorrecta o el dominio ya no existe.
- **Errores TLS** - el certificado de tu endpoint es inválido o ha expirado.
- **Conexión rechazada / timeout** - tu endpoint está caído.
- **Respuestas 5xx** - tu endpoint está activo pero dio error. El cuerpo de la respuesta (truncado) se registra.
- **Respuestas 4xx** - tu endpoint rechazó el payload. Si esto es intencional, añade el código a **No-retry status codes**.

### Pausar un webhook no saludable

Si un webhook falla consistentemente, la solución más limpia es eliminarlo (o vaciar temporalmente su lista de suscripciones de eventos). La plataforma no deshabilita automáticamente los webhooks que fallan - siguen reintentando hasta que la entrega se abandona.