Analytics es el panel de control multiagente. Se accede desde la página de AI Agents mediante la pestaña **Analytics** (para todo el tenant) o por agente mediante el botón **Analytics** en la fila de cada agente.

### Filtro

Un desplegable en la parte superior - **All agents** o un agente específico. El resto de la página se reajusta en consecuencia.

### Uso del presupuesto

Cuatro barras de progreso que muestran el gasto del período actual frente al límite:

- **Agent today** (cuando se filtra por un agente específico) - límite diario del agente.
- **Agent this month** - límite mensual del agente.
- **Account today** - límite diario del tenant.
- **Account this month** - límite mensual del tenant.

Cuando un límite no está configurado, la barra muestra "(no cap set)" y refleja el gasto real.

### Coste diario (últimos 30 días)

Una tabla con el coste por día en la moneda de tu tenant para el ámbito seleccionado. Útil para detectar:

- **Picos repentinos de coste** - normalmente por un bucle descontrolado o un comentario viral que dispara múltiples triggers.
- **Deriva de coste** - un aumento gradual del coste diario a medida que crece tu comunidad.

### Acciones realizadas

Un desglose de los tipos de acción durante el mes actual - "Wrote a comment: 47", "Marked a comment as spam: 12", y así sucesivamente. Útil para verificar que el agente está haciendo lo que esperabas.

### Triggers omitidos (este mes)

Recuentos agrupados por [motivo de descarte](#drop-reasons):

- Over agent daily / agent monthly / account daily / account monthly.
- Rate-limited.
- Concurrency saturated.

Si ves descartes aquí, tu agente está alcanzando un límite de presupuesto o de tasa y está perdiendo triggers en los que de otro modo habría actuado. Consulta [Drop Reasons](#drop-reasons).

### Dry-run vs live (este mes)

- **Enabled runs** - número de ejecuciones que realizaron acciones reales este mes.
- **Dry runs** - número de ejecuciones en modo dry-run este mes.

Una señal útil para el ajuste: un agente nuevo que aún no ha sido promovido a Enabled mostrará solo dry runs. Un agente en Enabled con recuentos en cero en esta sección está inactivo - o no se está activando, o está fuera del ámbito, o sus triggers no están configurados correctamente.

### Agentes principales por coste mensual

Cuando el filtro es **All agents**, la página lista los agentes ordenados por coste del mes en curso. Identificar tu agente más caro es el primer paso para optimizar costes - normalmente la respuesta es "ajustar sus [context options](#context-options)" o "bajar su [budget cap](#budgets-overview)".

### Agentes en o cerca de su límite

Desglose por agente de aquellos cuyo gasto está en o cerca de sus límites por agente en el período actual:

- **near cap** - por encima de un porcentaje configurable del límite.
- **over cap** - efectivamente limitado, con triggers `{count} dropped` en ese período.

Haz clic en el agente desde esta tabla para subir el límite, reducir el ámbito o pausarlo.

### Resumen de la cuenta

Cuando el filtro es **All agents**:

- **Triggers today** - recuento.
- **Triggers this month** - recuento.
- Para cada uno: un sufijo `dropped` que muestra cuántos fueron omitidos.

### Moneda

Todos los valores monetarios se muestran en la moneda de tu tenant.

### Lo que esta página no hace

- No muestra **desgloses de coste por acción** - esos están en [Run Detail View](#run-detail-view).
- No muestra **transcripciones** ni **respuestas del LLM**.
- No permite realizar acciones sobre los agentes - editar, pausar y eliminar se hacen desde la lista de agentes / página de edición.