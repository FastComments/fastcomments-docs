Hacer clic en **Ver** en una fila del [Historial de ejecuciones](#run-history) abre la página de detalle por ejecución. Aquí es donde lees el razonamiento del agente y juzgas sus decisiones.

### Parte superior: resumen de la ejecución

- **Agente** - qué agente se ejecutó.
- **Cuándo** - marca temporal.
- **Estado** - Iniciado / Éxito / Error, además del distintivo **Simulación** si procede.
- **Coste** - coste por ejecución en la moneda de tu tenant.
- **Coste por acción** - coste dividido por el número de acciones no pendientes, útil para detectar ejecuciones inusualmente caras.

### Acciones realizadas

Una lista, en orden, de cada llamada a herramienta que hizo la ejecución. Cada entrada muestra:

- **Etiqueta de acción** - "Escribió un comentario", "Marcó un comentario como spam", "Bloqueó a un usuario", y así sucesivamente. La etiqueta se mapea desde el tipo de acción (enumeración).
- **ID de referencia** - el ID del comentario, usuario o insignia afectado, mostrado como texto monoespaciado (no es un hipervínculo).
- **Razonamiento del agente** - la justificación que el agente proporcionó con la llamada.
- **Confianza** - la confianza autoevaluada del agente, mostrada como un porcentaje.
- **Insignia de aprobación pendiente** - si la acción está en cola en la [bandeja de aprobaciones](#approval-workflow) en lugar de ejecutarse.

Si la ejecución no tomó ninguna acción, la sección indica: "No se realizaron acciones durante esta ejecución."

### Transcripción LLM

Debajo de las acciones, la transcripción completa de la conversación del agente con el LLM:

- **Sistema** - el prompt del sistema (sufijo de la plataforma + tu prompt inicial + las directrices de la comunidad).
- **Usuario** - el mensaje de contexto que describe el disparador.
- **Asistente** - las respuestas del modelo, incluidas las llamadas a herramientas.
- **Herramienta** - el resultado de la herramienta devuelto al modelo (por ejemplo, lo que devolvió `search_memory`).

Los mensajes largos son plegables; haz clic en **Expandir** / **Contraer** para verlos.

### Lectura de transcripciones

La transcripción es la página más importante para ajustar. Cuando el agente tome una decisión con la que no estés de acuerdo, repásala para ver:

- Qué vio el modelo (el mensaje de contexto del Usuario).
- Qué decidió el modelo (las llamadas a herramientas del Asistente).
- Qué consideró el modelo (cualquier resultado de herramienta - por ejemplo, ¿el agente realmente llamó a `search_memory` y encontró algo antes de bloquear?).

Si el modelo comete de forma consistente el mismo tipo de error, edita el [prompt inicial](#personality-prompt) - o usa [Refinar prompts](#refining-prompts) a partir de una aprobación rechazada.

### Referencias de acciones

Los IDs de referencia se muestran como texto monoespaciado (no hipervínculos):

- Comentarios: el ID del comentario.
- Usuarios: el ID de usuario.
- Insignias: el ID de la insignia.

Puedes copiar el ID para buscar el registro afectado en la página de moderación/administración correspondiente.

### Qué falta en la simulación

Las ejecuciones en simulación muestran las mismas acciones, justificaciones y puntuaciones de confianza. La única diferencia es la insignia de Simulación en la fila de estado. Los IDs de referencia para comentarios / usuarios / insignias siguen mostrándose: el agente simplemente no los afectó.

### Errores

Para ejecuciones en estado `Error`, la página de detalle muestra el mensaje de error subyacente. Errores comunes:

- **No LLM API key configured** - mala configuración del tenant o de la plataforma.
- **LLM call timeout** - el proveedor de LLM fue lento o no estuvo disponible.
- **Tool dispatch failure** - el agente eligió una herramienta con argumentos inválidos (por ejemplo, un ID de comentario que ya no existe).
- **Budget exhausted mid-run** - se alcanzó el límite del agente mientras la ejecución estaba en curso. La ejecución se detuvo.

Los errores no revierten las acciones parciales: cualquier llamada a herramienta completada antes del error permanece.