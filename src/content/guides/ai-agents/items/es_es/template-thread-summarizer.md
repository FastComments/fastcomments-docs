**Template ID:** `thread_summarizer`

El Resumidor de hilos publica un resumen neutral de un solo párrafo al final de un hilo largo. Utiliza una demora de 30 minutos para que el hilo se asiente antes de que el agente lo revise.

### Prompt inicial incorporado

[inline-code-attrs-start title = 'Thread Summarizer Template Initial Prompt'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

La instrucción "do not editorialize" es esencial. Sin ella, el modelo tiende a enmarcarse con "in my view", lo que se lee mal bajo el nombre de tu cuenta.

### Desencadenantes

- **Se publica un nuevo comentario** (`COMMENT_ADD`).
- **Retraso del desencadenante**: 30 minutos (1800 segundos). Vea [Desencadenantes diferidos](#trigger-deferred-delay).

El retraso de 30 minutos significa que el agente se ejecuta una vez, media hora después de que llega un comentario, contra el aspecto que tenga el hilo en ese momento. No es "resumir en cada respuesta": la cola de desencadenadores diferidos consolida múltiples eventos de nuevo comentario en el mismo hilo, pero no los desduplica a través de ventanas separadas. Probablemente querrás **añadir una regla personalizada en tu prompt** como "do not post a new summary if the agent has already summarized this thread in the last 24 hours" y confiar en el contexto junto con las [herramientas de memoria](#tools-overview) del agente para que la haga cumplir.

### Herramientas permitidas

- [`write_comment`](#tools-overview) - publica el resumen en sí.
- [`pin_comment`](#tools-overview) - fija el resumen para que los lectores lo vean en la parte superior del hilo.
- [`unpin_comment`](#tools-overview) - desfija un resumen anterior del mismo agente antes de fijar el nuevo.

El resumidor no puede moderar ni interactuar con los usuarios.

### Fijar el resumen

El agente publica un nuevo comentario con `write_comment`, luego llama a `pin_comment` con el ID del comentario devuelto. En ejecuciones posteriores contra el mismo hilo, el prompt le indica que llame a `unpin_comment` sobre su resumen anterior primero: la plataforma en sí **no** hace cumplir una regla de un solo comentario fijado por hilo, así que dejar el resumen anterior fijado resultará en dos resúmenes fijados uno al lado del otro. Marca "Incluir el comentario padre y las respuestas anteriores en el mismo hilo" en [Opciones de contexto](#context-options) para que el agente pueda ver el resumen anterior fijado.

### Recomendaciones antes de ponerlo en marcha

- **Marca "Incluir el comentario padre y las respuestas anteriores en el mismo hilo"** en [Opciones de contexto](#context-options). Un resumidor sin contexto de hilo es inútil.
- **Ajusta la regla de tamaño mínimo del hilo.** "Fewer than 5 comments" es el valor predeterminado del prompt, pero en comunidades concurridas 10-20 es más apropiado. Edita el prompt directamente.
- **Restringe a patrones de URL específicos** si solo quieres resúmenes en páginas de formato largo, no en anuncios o páginas de producto. Vea [Alcance: filtros de URL y de localización](#scope-url-locale).
- **Controla los costos.** La resumización es la plantilla que más tokens consume porque lee todo el hilo en cada ejecución. Establece un [presupuesto diario](#budgets-overview) ajustado antes de cambiar a Enabled.

### Evitar resúmenes repetidos

El agente tiene acceso a [`save_memory`](#tools-overview) y [`search_memory`](#tools-overview): puedes ampliar el prompt para indicarle que registre notas como "resumido {thread urlId}" y que las busque antes de publicar de nuevo. La memoria se comparte entre todos los agentes en tu tenant.