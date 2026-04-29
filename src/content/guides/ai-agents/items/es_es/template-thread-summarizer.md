**Template ID:** `thread_summarizer`

El resumidor de hilos publica un resumen neutral de un solo párrafo al final de un hilo largo. Usa un retraso de 30 minutos para que el hilo se asiente antes de que el agente lo analice.

El prompt integrado instruye al agente a no hacer comentarios editoriales: esto es fundamental. Sin ello, el modelo tiende al encuadre "en mi opinión", que se lee mal bajo el nombre de visualización de su cuenta.

### Disparadores

- **Nuevo comentario publicado** (`COMMENT_ADD`).
- **Retraso del disparador**: 30 minutos (1800 segundos). Ver [Disparadores diferidos](#trigger-deferred-delay).

El retraso de 30 minutos significa que el agente se ejecuta una vez, media hora después de que llega un comentario, contra el estado que tenga el hilo en ese momento. No es "resumir en cada respuesta": la cola de disparadores diferidos aglutina múltiples eventos de nuevo comentario en el mismo hilo, pero no los desduplica a través de ventanas separadas. Probablemente querrá **añadir una regla personalizada en su prompt** como "do not post a new summary if the agent has already summarized this thread in the last 24 hours" y confiar en el contexto más las [herramientas de memoria](#tools-overview) del agente para aplicarla.

### Herramientas permitidas

- [`write_comment`](#tools-overview) - publica el resumen en sí.
- [`pin_comment`](#tools-overview) - fija el resumen para que los lectores lo vean en la parte superior del hilo.
- [`unpin_comment`](#tools-overview) - desfija un resumen anterior del mismo agente antes de fijar el nuevo.

El resumidor no puede moderar ni interactuar con los usuarios.

### Fijar el resumen

El agente publica un nuevo comentario con `write_comment`, luego llama a `pin_comment` con el ID del comentario devuelto. En ejecuciones posteriores contra el mismo hilo, el prompt le instruye que llame primero a `unpin_comment` sobre su resumen anterior: la plataforma en sí **no** aplica una regla de "un solo comentario fijado" por hilo, por lo que dejar el resumen anterior fijado dará lugar a dos resúmenes fijados uno junto al otro. Marque "Include parent comment and prior replies in the same thread" en [Opciones de contexto](#context-options) para que el agente pueda ver el resumen fijado anterior.

### Adiciones recomendadas antes de ponerlo en producción

- **Marque "Include parent comment and prior replies in the same thread"** en [Opciones de contexto](#context-options). Un resumidor sin contexto de hilo es inútil.
- **Ajuste la regla de tamaño mínimo del hilo.** "Fewer than 5 comments" es el valor por defecto del prompt, pero en comunidades con mucho tráfico 10-20 es más apropiado. Edite el prompt directamente.
- **Restrinja a patrones de URL específicos** si solo quiere resúmenes en páginas de formato largo, no en anuncios o páginas de producto. Vea [Ámbito: filtros de URL y localización](#scope-url-locale).
- **Vigile los costes.** La resumición es la plantilla que más tokens consume porque lee todo el hilo en cada ejecución. Establezca un [presupuesto diario](#budgets-overview) ajustado antes de cambiar a "Enabled".

### Evitando resúmenes repetidos

El agente tiene acceso a [`save_memory`](#tools-overview) y [`search_memory`](#tools-overview): puede ampliar el prompt para indicarle que registre notas como "resumido {thread urlId}" y las consulte antes de publicar de nuevo. La memoria se comparte entre todos los agentes de su tenant.