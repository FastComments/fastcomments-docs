**Refinar el prompt** es el flujo de trabajo para editar el [prompt inicial](#personality-prompt) de un agente en respuesta a decisiones específicas con las que no estás de acuerdo. Se lanza desde la [bandeja de aprobaciones](#approval-workflow).

### Cuándo usarlo

Cuando te encuentras rechazando el mismo tipo de aprobación una y otra vez - "el agente sigue queriendo banear a personas por usar lenguaje fuerte sin un objetivo" - el prompt del agente es la palanca para solucionarlo. Refinar el prompt es una forma guiada de:

1. Elegir una aprobación específica que representa la mala decisión.
2. Editar el prompt con el contexto completo de lo que hizo el agente y por qué.
3. Guardar el nuevo prompt en el agente.

El resultado es un agente que, de ahora en adelante, probablemente no tomaría la misma decisión.

### Lanzamiento del flujo

Desde la bandeja de aprobaciones en `/auth/my-account/ai-agent-approvals`:

1. Abre una aprobación **rechazada**. La ruta rechaza estrictamente todo excepto REJECTED - las aprobaciones pending y execution-failed no son elegibles.
2. Haz clic en **Refinar prompt**.

Llegas a la interfaz de refinamiento de prompt en `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Qué muestra la página

- **La aprobación** - el `toolName` del agente y la `justification` para la decisión rechazada (aquí no se muestra la transcripción completa del LLM).
- **El prompt actual** - el [prompt inicial](#personality-prompt) guardado del agente.
- **Un campo de comentarios** - escribes **feedback** describiendo lo que debería cambiar (hasta 2000 caracteres). El LLM genera el nuevo prompt propuesto a partir de tu feedback.
- **Diff en línea unificado** - un único diff en línea entre el prompt actual y el propuesto (rojo para lo eliminado, verde para lo añadido).

El contexto de la aprobación queda fijado en la parte superior para que puedas seguir refiriéndote al "caso que estoy arreglando" mientras editas.

### Guardar

Guardar actualiza el campo `initialPrompt` del agente. Las ejecuciones pasadas (y las aprobaciones pasadas) no se vuelven a ejecutar de forma retroactiva: el nuevo prompt solo afecta a los desencadenantes futuros. Si quieres verificar que el nuevo prompt soluciona el problema, ejecuta una [ejecución de prueba / reproducción](#test-runs-replays) contra los últimos 7 días y observa si el nuevo prompt todavía produciría la aprobación rechazada.

### Lo que el flujo no hace

- No edita las **directrices de la comunidad** - ese campo tiene su propio editor en el formulario principal de edición del agente.
- No edita **triggers**, **allowed tools**, ni **approval gating** - esos permanecen en el formulario principal de edición.
- No versiona el prompt con reversión. El prompt anterior no se almacena en una colección de historial separada. Si necesitas revertir, copia el prompt actual en tu propio sistema de seguimiento antes de editar.

### Por qué emparejar refinar con reproducción

Editar un prompt sin probar el resultado es cuestión de fe. El ciclo recomendado:

1. Rechaza una aprobación.
2. Refina el prompt.
3. Ejecuta una [ejecución de prueba](#test-runs-replays) contra los últimos 7 días.
4. Mira la pestaña **Deltas**. ¿El nuevo prompt movió la mala decisión de "haría" a "no haría"? ¿Accidentalmente movió también decisiones buenas fuera?
5. Itera.

Tres o cuatro ciclos de refinar + reproducir suelen ser suficientes para obtener un prompt estable para un agente de moderación.

### Alternativa de edición directa

No tienes que usar Refine Prompt: también puedes editar el agente en el formulario principal de edición. La única ventaja de Refine Prompt es que fija un caso fallido específico para que no pierdas de vista lo que estás arreglando.