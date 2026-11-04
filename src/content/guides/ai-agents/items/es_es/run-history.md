El Historial de Ejecuciones es el registro por agente de cada disparador que se ejecutó. Se accede desde la página de la lista de agentes mediante el botón **Runs**, o directamente en `/auth/my-account/ai-agents/{agentId}/runs`.

### Qué hay en la página

Una tabla paginada con una fila por ejecución:

| Columna | Significado |
|---|---|
| Date | Cuándo se disparó el disparador (o cuándo se ejecutó el disparador diferido). |
| Status | **Started**, **Success** o **Error**. La insignia **Dry Run** se muestra junto a ella si la ejecución estaba en modo de prueba (dry-run). |
| Cost | Coste por ejecución en la moneda de tu tenant. Vacío para las ejecuciones en curso (Started). |
| Actions | El número de llamadas a herramientas en la ejecución. |
| Details | Un botón **View** que abre la [Vista de Detalle de Ejecución](#run-detail-view). |

### Significado de los estados

- **Started** - la ejecución está en progreso, o murió antes de completarse. Una ejecución atascada en "Started" durante un tiempo inusualmente largo suele representar un timeout en la llamada al LLM.
- **Error** - la ejecución se completó pero falló en algún punto - una llamada al LLM devolvió un error, falló el envío de una herramienta, etc. La vista de detalle contiene el error específico.
- **Success** - la ejecución se completó sin errores. El agente puede haber realizado cero, una o muchas acciones.

### Estado vacío

Cuando un agente no tiene ejecuciones, la página muestra: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

Esa última parte es intencional - el [flujo de ejecución de prueba](#test-runs-replays) es la forma recomendada de poblar el Historial de Ejecuciones en un agente nuevo.

### Qué no está en la página de historial de ejecuciones

- **Disparadores en vivo que nunca se enviaron** - un disparador descartado por presupuesto, alcance o limitación de tasa no aparece en esta página. Estos se muestran en la [página de Analytics](#analytics-page) bajo "Triggers skipped".
- **Aprobaciones** - las aprobaciones pendientes de acciones tomadas en esta ejecución residen en la [bandeja de aprobaciones](#approval-workflow). La acción aparece en la vista de detalle de la ejecución como **Pending approval**.

### Retención

Los registros individuales de ejecución se conservan durante 90 días, después de los cuales la ejecución desaparece del historial. Los costes y los recuentos de disparadores siguen acumulándose en los resúmenes analíticos a largo plazo, por lo que la [página de Analytics](#analytics-page) sigue mostrando totales históricos más allá de esa ventana.

### Repeticiones (Replays)

Las ejecuciones producidas por replays se excluyen de la vista de ejecuciones en vivo por defecto. La página [Test Runs (Replays)](#test-runs-replays) es donde puedes verlas.

### Filtrado entre agentes

La tabla de ejecuciones es por agente. No existe una vista de ejecuciones entre agentes - la [página de Analytics](#analytics-page) es el resumen entre agentes. Si necesitas inspeccionar ejecuciones a través de múltiples agentes, los eventos `trigger.succeeded` y `trigger.failed` de los [Webhooks](#webhooks-overview) son lo que reenviarías a tu propio sistema.