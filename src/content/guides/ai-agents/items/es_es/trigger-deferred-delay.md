Por defecto, un agente se ejecuta **inmediatamente** después de que su disparador se activa. El campo **Retraso antes de ejecutar** en el formulario de edición cambia eso: la plataforma encola el disparador y ejecuta el agente en el momento programado.

### Cuándo usar un retraso

- **Disparadores de umbral de banderas** - las banderas a menudo llegan en ráfagas. Un retraso de 10-30 minutos permite que la situación se estabilice para que el agente actúe sobre el recuento final de banderas en lugar del momento de llegada.
- **Disparadores de umbral de votos** - misma lógica, particularmente para brigadas de votos negativos.
- **Resumen de hilos** - la [plantilla del Resumidor de Hilos](#template-thread-summarizer) tiene por defecto un retraso de 30 minutos para que resuma una conversación que haya tenido tiempo de desarrollarse, no un hilo con dos respuestas.
- **Periodo de enfriamiento / re-evaluación** - "24 horas después de que un comentario se bloquee, considerar si desbloquearlo."

### Configuración

- **Campo**: Retraso antes de ejecutar.
- **Rango**: 0 a 2.592.000 segundos (30 días).
- **Unidades**: Segundos, minutos, horas o días.

### Idempotencia

La cola diferida no elimina disparadores duplicados. Dos banderas que lleguen con 1 segundo de diferencia en un agente con un retraso de 30 minutos programarán ambas una ejecución 30 minutos después, y el agente se ejecutará **dos veces**, ambas veces contra (principalmente) el mismo contexto. Si quieres semántica de como mucho-una-ejecución-por-ventana, el agente debe aplicarlo: típicamente escribiendo una [nota de memoria](#tools-overview) en la primera ejecución y comprobándola en las ejecuciones subsiguientes.

### Nota de coste

Los disparadores diferidos se registran **antes** de ejecutarse. Una ráfaga de disparadores en un agente con alto retraso puede acumularse en la cola sin gastar tokens; el coste se paga solo cuando el cron los despacha. Usa [Historial de ejecuciones](#run-history) y [Razones de descarte](#drop-reasons) para ver con qué frecuencia los disparadores diferidos realmente se ejecutan frente a ser descartados en tiempo de ejecución por motivos de presupuesto.

### La reproducción no respeta el retraso

La función de [Ejecuciones de prueba (Reproducciones)](#test-runs-replays) ejecuta el agente inmediatamente contra comentarios históricos: no espera el retraso configurado. Considéralo una característica: las reproducciones sirven para previsualizar lo que el agente **haría** dado el contexto, no para reproducir la programación en tiempo real.