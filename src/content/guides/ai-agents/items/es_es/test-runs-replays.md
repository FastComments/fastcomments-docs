Un **test run** (también llamado **replay**) ejecuta el agente contra una ventana de comentarios históricos **sin tomar acciones reales**. Es la forma más rápida de previsualizar el comportamiento del agente antes de ponerlo en producción.

Accesible desde la página de lista de agentes mediante el botón **Test run** en la fila de cada agente.

### Qué hace

La plataforma:

1. Selecciona una muestra de comentarios históricos que coinciden con el alcance del agente, en la ventana que elijas.
2. Para cada comentario, ejecuta el agente de extremo a extremo como si el comentario acabara de publicarse: mismo contexto, misma llamada LLM, misma selección de herramientas, mismas justificaciones y puntuaciones de confianza.
3. Registra cada ejecución como dry-run, etiquetada para que permanezca agrupada con la replay de la que provino y excluida de las vistas de ejecuciones en vivo.
4. **Compara** el veredicto del agente con lo que realmente sucedió con el comentario: si luego fue aprobado, marcado como spam, eliminado, bloqueado por el motor de spam, etc.

El resultado es una diferencia por comentario: "El agente en replay marcaría esto como spam, pero el comentario actualmente está aprobado y limpio."

### Configuración

La página de test-run tiene una sola entrada:

- **Days of historical comments to evaluate** - un campo numérico `days` entre 1 y 90. Los comentarios más antiguos no son elegibles.

El tamaño de la muestra y el límite máximo no están expuestos en la UI: ambos son valores predeterminados del servidor aplicados por plan. La página muestra campos informativos:

- **Matching comments in window** - cuántos comentarios se considerarían.
- **Up to N comments from this window will be processed** - el tamaño de muestra efectivo dado el límite del lado del servidor.
- **Estimated cost** - en la moneda de tu tenant.

### Límite de frecuencia

Cada usuario está limitado a **10 test runs por 24 horas** (rate-limited vía la clave `replay-create:${requestedBy}`). El botón muestra un tooltip cuando has alcanzado el límite ("You've reached 10 test runs in the last 24 hours.").

### Concurrencia

Solo puede haber una replay activa por agente a la vez. Iniciar una segunda replay mientras una está en curso te redirige a la que está en vuelo.

### Leer resultados

Cuando la replay termina, la página de resultados muestra pestañas:

- **Deltas** (activo por defecto) - el veredicto del agente en replay difiere de la realidad. (Lo más interesante: "el agente habría marcado este comentario como spam, pero el comentario fue aprobado y está bien".)
- **Matches** - el veredicto del agente en replay coincide con lo que realmente sucedió. (Tranquilizador: el agente está de acuerdo con la realidad.)
- **No action** - el agente en replay decidió no hacer nada. (A veces es la respuesta correcta; otras veces el agente pasó por alto algo.)
- **All** - todos los resultados sin importar la clasificación.

Para cada comentario en cualquier pestaña:

- **Prior outcome** - la clasificación de lo que realmente sucedió: **POSITIVO**, **NEGATIVO** o **INDETERMINADO**, con **Evidencia** ("Comment marked deleted at {date}", "Engine: bayes", y así sucesivamente).
- **Replay agent would** - la acción que el agente eligió.
- **Why** - la justificación.
- **Confidence** - mostrada como un porcentaje.

### Por qué los replays fuerzan dry-run

Una replay contra un comentario que fue eliminado hace cuatro meses no debería eliminarlo retroactivamente: ya está eliminado. Una replay contra un comentario que ahora el agente quiere aprobar no debería cambiar el estado actual del comentario. Replay es una herramienta de previsualización. Forzar dry-run es lo que hace seguro ejecutar una replay contra cualquier ventana histórica.

### Reproducibilidad

Las replays congelan la configuración del agente en el momento en que se inició la replay. Las ediciones posteriores al agente no cambian los resultados de la replay: la página de resultados permanece estable como registro de lo que *esa* versión del agente habría hecho.

### Cuando los presupuestos detienen una replay

Las replays están sujetas a:

- Su propio **hard cap** (establecido en el formulario de replay).
- Los topes de presupuesto diarios y mensuales del agente.
- Los topes de presupuesto diarios y mensuales del tenant.

El primero que se alcance aborta la replay con un código de error específico. Cualquier resultado por comentario producido antes del aborto se conserva en [Historial de ejecuciones](#run-history).

### Cómo se ejecutan las replays

Las replays se ejecutan en segundo plano, no de forma síncrona. Después de hacer clic en "Start test run", la replay se encola y un worker la toma. Una replay larga puede durar varios minutos. La página de resultados consulta y muestra el progreso (conteo procesado, gasto hasta el momento) mientras avanza.

Si un worker muere a mitad de la replay, la plataforma vuelve a encolar automáticamente la replay para que se reanude en la siguiente pasada. Un breve tropiezo nunca deja una replay huérfana.

### Qué no hace la replay

- **No respeta los [trigger delays](#trigger-deferred-delay).** Las replays se ejecutan de inmediato, no 30 minutos después.
- **No escribe en la memoria.** Los agentes en replay no guardan notas de memoria, incluso si su lógica normalmente lo haría.
- **No dispara webhooks.** Los triggers producidos por la replay no generan eventos de webhook `trigger.succeeded` / `trigger.failed`.
- **No excluye comentarios ya replayed.** Ejecutar una segunda replay contra la misma ventana cubre los mismos comentarios.

### Véase también

- [Refining Prompts](#refining-prompts) - el flujo de trabajo de edición iterativa que funciona bien con las replays.
- [Dry-Run Mode](#dry-run-mode) - la misma idea, pero contra tráfico en vivo.