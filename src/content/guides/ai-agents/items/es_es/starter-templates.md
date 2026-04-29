FastComments ofrece cuatro plantillas iniciales para que no tengas que escribir un agente funcional desde cero. Se pueden encontrar en la [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) haciendo clic en **Explorar plantillas**.

Cuando eliges una plantilla:

1. El agente se crea con **Estado: Modo de prueba** y un nombre interno basado en la plantilla (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Si ese nombre ya existe en tu tenant, se añade un sufijo numérico.
2. Aterrizas directamente en el formulario de edición con todo rellenado: prompt, triggers, acciones permitidas y cualquier umbral. Un banner en la parte superior dice "Se ha creado a partir de la plantilla {templateName}. Revise las configuraciones a continuación y luego cambie el estado a Habilitado cuando esté listo."
3. Nada está habilitado aún. El agente no actuará hasta que guardes y mantengas el modo de prueba activado (para observar) o lo pongas en Habilitado.

### Las cuatro plantillas

- **[Moderator](#template-moderator)** - revisa comentarios nuevos y señalados, advierte a los infractores primerizos y escala al baneo solo después de una advertencia. Se activa con nuevos comentarios y con cruces del umbral de banderas (umbral de banderas predeterminado: 3). Herramientas permitidas: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - responde con calidez a quienes comentan por primera vez con una bienvenida corta y personal. Se activa con new-user-first-comment. Herramienta permitida: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - fija comentarios de nivel superior sustantivos una vez que superan un umbral de votos (predeterminado: 10), desanclando primero el comentario previamente fijado. Se activa con cruces del umbral de votos. Herramientas permitidas: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publica un resumen neutral de un solo párrafo en hilos largos después de un retraso y luego lo fija. Se activa con comentarios nuevos con una demora de 30 minutos para que el hilo se asiente antes de resumir. Herramientas permitidas: `write_comment`, `pin_comment`, `unpin_comment`.

### Personalizar una plantilla

Las plantillas son puntos de partida, no contratos. Se espera que:

- Ajustes el **Initial prompt** para que coincida con la voz de tu comunidad.
- Añadas o elimines **Triggers** para adaptar la frecuencia con la que debe ejecutarse el agente.
- Añadas **Approvals** para cualquier acción sensible: recomendamos encarecidamente que `ban_user` requiera aprobación en plantillas de estilo moderador.
- Añadas **Community guidelines** para que el agente aplique tu política escrita de forma consistente. Véase [Community Guidelines](#community-guidelines).
- Establezcas **Budgets** por agente adecuados a la cantidad de triggers que esperas.

La plantilla es solo un vehículo que precompleta valores predeterminados sensatos; una vez guardado, el agente es tuyo.