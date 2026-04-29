Hay cuatro tipos de eventos webhook de agente. Cada evento tiene un valor numérico de enum (usado en los payloads) y un nombre canónico en cadena (usado en el campo `event` del sobre y en el encabezado HTTP `X-FastComments-Agent-Event`).

| Nombre del evento | Enumeración | Se dispara cuando |
|---|---|---|
| `trigger.succeeded` | 0 | Una ejecución del agente finaliza con estado `SUCCESS`. |
| `trigger.failed` | 1 | Una ejecución del agente finaliza con estado `ERROR`. |
| `approval.requested` | 2 | Se encola una aprobación en estado `PENDING`. |
| `approval.decided` | 3 | Una aprobación pasa a `APPROVED`, `REJECTED` o `EXECUTION_FAILED`. |

### `trigger.succeeded`

Se dispara después de que la ejecución del agente termina sin error. El campo `data` del payload incluye:

- `triggerId` - el ID único de la ejecución.
- `triggerType` - la [enumeración de motivos del trigger](#triggers-overview) que inició la ejecución.
- `status` - `SUCCESS` (cadena).
- `tokensUsed` - tokens consumidos en esta ejecución.
- `wasDryRun` - true si el agente estaba en [modo dry-run](#dry-run-mode).
- `actions` - matriz de registros `TenantAgentAction` (ver [Payloads del webhook](#webhook-payloads)).
- `commentId`, `url`, `urlId` - si el trigger los tenía.

Si la ejecución realizó cero acciones, el array `actions` está vacío: esto es una ejecución exitosa de "el agente decidió no hacer nada", lo cual es útil saberlo.

### `trigger.failed`

Se dispara cuando una ejecución falla. Misma forma de payload que `trigger.succeeded`, con `status: 'ERROR'` y un campo adicional `errorMessage` que describe qué salió mal. Errores posibles incluyen fallos en llamadas LLM, fallos al despachar herramientas y agotamiento del presupuesto a mitad de ejecución.

`actions` aún puede contener entradas para llamadas a herramientas que se completaron antes del error.

### `approval.requested`

Se dispara en el momento en que una aprobación se encola en estado `PENDING`. El payload incluye:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - los argumentos de la herramienta **pasados literalmente** desde la llamada LLM. La forma es por herramienta y no es un contrato público estable: el esquema puede cambiar conforme se añadan nuevas herramientas.
- `createdAt`.
- `justification`, `confidence` - si el agente los proporcionó.
- `contextSnapshot` - el contexto del comentario / página al que se refiere la aprobación.

Útil para reenviar aprobaciones pendientes a un canal de chat ops: un bot de Slack suscrito a `approval.requested` puede publicar la acción y el razonamiento en un canal de moderación para una revisión rápida.

### `approval.decided`

Se dispara cuando una aprobación sale de `PENDING`. El payload incluye:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, o `EXECUTION_FAILED`.
- `decidedBy` - el ID de usuario del moderador que decidió.
- `decidedAt` - cuándo decidió.
- `executedAt` - si está `APPROVED`, cuándo la plataforma ejecutó la acción aprobada.
- `executionResult` - si está `APPROVED`, una cadena que describe el resultado del ejecutor.
- `contextSnapshot` - el contexto del comentario / página.

Este evento cubre todos los resultados de decisión:

- **Aprobado + ejecutado correctamente** -> `status: APPROVED`, `executedAt` establecido, `executionResult` contiene el mensaje de éxito.
- **Aprobado + el ejecutor falló** -> `status: EXECUTION_FAILED`, `executedAt` establecido, `executionResult` describe la falla.
- **Rechazado** -> `status: REJECTED`, `executedAt` es null, `executionResult` es null.

### Encabezado

Cada entrega incluye un encabezado HTTP `X-FastComments-Agent-Event` con el nombre canónico en cadena del evento (`trigger.succeeded`, etc.). Útil si tu endpoint es una única URL que maneja múltiples tipos de evento.

### Véase también

- [Payloads del webhook](#webhook-payloads) para los esquemas completos de payload por evento.
- [Firma de webhooks](#webhook-signing) para el esquema HMAC.
- [Reintentos de webhooks](#webhook-retries) para la semántica de entrega.