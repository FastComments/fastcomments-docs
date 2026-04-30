FastComments incluye cinco plantillas iniciales para que no tengas que escribir un agente funcional desde cero. Se pueden acceder desde la [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) haciendo clic en **Browse templates**.

Cuando eliges una plantilla:

1. El agente se crea con **Estado: Modo de prueba** y un nombre interno basado en la plantilla (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Si ese nombre ya existe en tu tenant, se añade un sufijo numérico.
2. Llegas directamente al formulario de edición con todo rellenado: prompt, triggers, acciones permitidas y cualquier umbral. Un banner en la parte superior indica "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nada está habilitado todavía. El agente no actuará hasta que guardes y mantengas el modo de prueba activado (para observar) o lo cambies a Habilitado.

### Las cinco plantillas

- **[Moderador](#template-moderator)** - revisa comentarios nuevos y marcados, advierte a los infractores primerizos y escala a ban solo después de una advertencia. Se activa con comentarios nuevos y con cruces de flag-threshold (flag threshold predeterminado: 3). Herramientas permitidas: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Saludo de bienvenida](#template-welcome-greeter)** - responde con calidez a los usuarios que comentan por primera vez con una bienvenida breve y personal. Se activa con new-user-first-comment. Herramienta permitida: `write_comment`.

- **[Fijador de comentarios destacados](#template-top-comment-pinner)** - fija comentarios sustantivos de primer nivel una vez que superan un umbral de votos (predeterminado: 10), desanclando primero el comentario previamente fijado. Se activa con vote-threshold crossings. Herramientas permitidas: `pin_comment`, `unpin_comment`.

- **[Resumidor de hilo](#template-thread-summarizer)** - publica un resumen neutral de un solo párrafo en hilos largos después de un retraso y luego lo fija. Se activa con comentarios nuevos con un aplazamiento de 30 minutos para que el hilo se asiente antes de resumir. Herramientas permitidas: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Detector de gaslighting](#template-gaslight-detector)** - vigila las ediciones de comentarios en busca de reescrituras a mitad del hilo que distorsionen las respuestas, restaura el texto original y envía un mensaje directo al autor. Se activa con ediciones de comentarios. Herramientas permitidas: `edit_comment`, `warn_user`, `send_dm`.

### Personalizar una plantilla

Las plantillas son puntos de partida, no contratos. Se espera que:

- Ajustes el **Prompt inicial** para que coincida con la voz de tu comunidad.
- Añadas o quites **Disparadores** para adaptar con qué frecuencia debe ejecutarse el agente.
- Añadas **Aprobaciones** para cualquier acción sensible: recomendamos encarecidamente requerir aprobación para `ban_user` en plantillas de estilo moderador.
- Añadas **Pautas de la comunidad** para que el agente aplique coherentemente tu política escrita. Consulta [Pautas de la comunidad](#community-guidelines).
- Establezcas **Presupuestos** por agente apropiados según la cantidad de disparadores que esperes.

La plantilla es solo un vehículo que rellena valores predeterminados sensatos; una vez guardada, el agente es tuyo.

---