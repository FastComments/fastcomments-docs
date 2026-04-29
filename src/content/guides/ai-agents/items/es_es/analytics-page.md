Analytics es el panel transversal entre agentes. Accesible desde la página Agentes de IA mediante la pestaña **Analytics** (a nivel del tenant) o por agente mediante el botón **Analytics** en la fila de cada agente.

### Filter

Un desplegable en la parte superior: **All agents** o un agente específico. El resto de la página se reescopa en consecuencia.

### Budget usage

Cuatro barras de progreso que muestran el gasto del periodo actual frente al límite:

- **Agent today** (cuando se filtra por un agente específico) - límite diario por agente.
- **Agent this month** - límite mensual por agente.
- **Account today** - límite diario del tenant.
- **Account this month** - límite mensual del tenant.

Cuando no hay un límite establecido, la barra muestra "(no cap set)" y enseña el gasto bruto.

### Daily cost (last 30 days)

Una tabla del coste por día en la moneda de su tenant para el alcance seleccionado. Útil para detectar:

- **Picos bruscos de coste** - normalmente por un bucle descontrolado o un comentario viral que dispare acciones en cadena.
- **Deriva de coste** - aumento gradual del coste diario a medida que crece su comunidad.

### Actions taken

Un desglose de tipos de acciones durante el mes en curso: "Wrote a comment: 47", "Marked a comment as spam: 12", y así sucesivamente. Útil para comprobar que el agente está haciendo lo que esperaba.

### Triggers skipped (this month)

Recuentos agrupados por [razón de descarte](#drop-reasons):

- Por superar el límite diario/mensual del agente o el límite diario/mensual de la cuenta.
- Limitado por tasa.
- Concurrencia saturada.

Si ve descartes aquí, su agente está alcanzando un presupuesto o un límite de tasa y se está perdiendo triggers en los que de otro modo habría intervenido. Vea [Razones de descarte](#drop-reasons).

### Dry-run vs live (this month)

- **Enabled runs** - conteo de ejecuciones que realizaron acciones reales este mes.
- **Dry runs** - conteo de ejecuciones en modo dry-run este mes.

Una señal útil de ajuste: un agente completamente nuevo que aún no ha sido promovido a Habilitado mostrará solo dry runs. Un agente en Habilitado con todos los contadores a cero en esta sección está inactivo: o no se le dispara, o está siendo excluido por alcance, o sus triggers no están configurados correctamente.

### Top agents by monthly cost

Cuando el filtro es **All agents**, la página lista agentes ordenados por coste acumulado en el mes. Identificar su agente más caro es el primer paso en la optimización de costes: normalmente la respuesta es "ajustar sus [opciones de contexto](#context-options)" o "reducir su [límite de presupuesto](#budgets-overview)".

### Agents at or near their cap

Desglose por agente de aquellos cuyo gasto está en o cerca de sus límites por agente en el periodo actual:

- **near cap** - por encima de un porcentaje configurable del límite.
- **over cap** - realmente limitado, con `{count} dropped` triggers en ese periodo.

Haga clic en el agente desde esta tabla para aumentar el límite, reducir el alcance o pausarlo.

### Account summary

Cuando el filtro es **All agents**:

- **Triggers today** - conteo.
- **Triggers this month** - conteo.
- Para cada uno: un sufijo `dropped` que muestra cuántos fueron omitidos.

### Currency

Todos los valores monetarios se muestran en la moneda de su tenant.

### What this page does not do

- No muestra **desgloses de coste por acción** - esos están en la [Vista de detalle de ejecución](#run-detail-view).
- No muestra **transcripciones** ni **respuestas de LLM**.
- No le permite actuar sobre agentes: editar, pausar o eliminar se hace desde la lista de agentes / la página de edición.