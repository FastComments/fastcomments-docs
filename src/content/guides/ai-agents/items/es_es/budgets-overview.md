Cada agente tiene límites de gasto. La plataforma deja de despachar al agente cuando se alcanza un límite y se reanuda una vez que finaliza el período.

### Dos ámbitos, dos períodos

Hay cuatro límites en total: dos ámbitos (por agente, por inquilino) cruzados con dos períodos (diario, mensual).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

Un disparador solo se despacha si **los cuatro límites** lo permiten. El primer límite que se agote será el que haga que el disparador sea descartado.

### Moneda

Los presupuestos por agente se introducen en la moneda de tu cuenta.

### Qué ocurre cuando se alcanza un límite

- El disparador se registra como **descartado** con un [motivo de descarte](#drop-reasons) como `agentDaily` o `tenantMonthly`.
- El recuento de descartados aparece en la [Página de análisis](#analytics-page) bajo "Disparadores omitidos (este mes)".
- No se realiza ninguna llamada a LLM; no se gastan tokens en el propio disparador descartado.
- El estado del agente no cambia: simplemente no puede despacharse hasta que se reinicie el período.

### Reinicio del período

- Los límites **diarios** se restablecen a la medianoche UTC.
- Los límites **mensuales** se restablecen al inicio de cada mes del calendario, en UTC.

No hay traspaso del presupuesto no usado al siguiente período.

### Límite duro vs advertencias suaves

Los límites son **estrictos**. No existe un modo de "exceder en un 10% con advertencia". Cuando se alcanza el límite, el despacho se detiene.

La parte "suave" son los correos electrónicos de [Alertas de presupuesto](#budget-alerts): recibes un correo en umbrales configurables (por defecto 80% y 100%) para que puedas aumentar el límite antes de que el tráfico empiece a caer.

### Dónde consultar el uso actual

- [Página de análisis](#analytics-page) - uso del presupuesto por agente y a nivel de inquilino con marcadores de límite.
- La sección **Estadísticas** del formulario de edición del agente.
- La vista de lista (el recuento de aprobaciones pendientes y ejecuciones recientes aparece en la tarjeta del agente).

### Elegir un presupuesto

Algunas reglas generales:

- **Un agente nuevo** - determina el presupuesto. Observa el [historial de ejecuciones](#run-history) durante una semana. Ajusta en base al coste observado por ejecución × el volumen de disparadores esperado.
- **Un agente de alto volumen** (p. ej., disparador de nuevo comentario en un sitio muy concurrido) - el límite diario es lo que atrapa un bucle descontrolado. Elige un límite diario que sea 2-3x tu gasto diario previsto para que un día normal y ocupado encaje holgadamente por debajo.
- **Un agente resumidor o con mucho contexto** - el coste por ejecución es alto. Establece un límite diario más estricto para evitar que un mal día agote el mensual.

### Omisión del presupuesto para reproducciones

[Ejecuciones de prueba / reproducciones](#test-runs-replays) están sujetas a su **propio** límite estricto (establecido en el formulario de reproducción, separado de los límites diarios/mensuales del agente), Y a los límites del agente y del inquilino. El que se alcance primero detiene la reproducción.

### Ver también

- [Alertas de presupuesto](#budget-alerts) para las notificaciones por correo.
- [Modelo de costes](#cost-model) para cómo la plataforma convierte tokens a dólares.
- [Motivos de descarte](#drop-reasons) para la lista completa de razones por las que un disparador no se ejecuta.