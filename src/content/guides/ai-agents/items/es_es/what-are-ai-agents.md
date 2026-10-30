Un **AI Agent** es un trabajador autónomo, limitado a su inquilino de FastComments, que observa los eventos en su comunidad y actúa en su nombre.

Cada agente tiene tres cosas que controla:

1. **Una personalidad.** Un mensaje inicial de texto libre que define el tono, el rol y el estilo de toma de decisiones (“Eres un cordial saludo de la comunidad”, “Aplicarás las reglas de la comunidad pero tiendes a advertir antes de prohibir”, etc.).
2. **Uno o más disparadores.** Una lista de eventos que despiertan al agente: un nuevo comentario, un comentario que supera un umbral de votos o denuncias, una acción de moderador, el primer comentario de un usuario en el sitio, entre otros. La lista completa está en [Trigger Events Overview](#triggers-overview).
3. **Una lista blanca de herramientas.** Lo que el agente tiene permitido hacer: publicar un comentario, votar, fijar, bloquear, marcar como spam, prohibir a un usuario, advertir por DM, otorgar una insignia, enviar correo electrónico, guardar y buscar en una memoria compartida. La lista completa está en [Allowed Tool Calls Overview](#tools-overview).

Cuando se dispara un disparador, el agente recibe un mensaje de contexto que describe lo ocurrido (el comentario, la página, contexto opcional de hilo/usuario/página) y se le muestra su mensaje inicial junto con las directrices de su comunidad. Luego llama a herramientas para actuar, registrando una justificación y una puntuación de confianza con cada llamada.

### Los agentes se ejecutan de forma asíncrona

Los agentes **nunca bloquean la acción del usuario que los activó**. Un lector publica un comentario, el comentario se guarda y se muestra en el hilo, la respuesta se devuelve, y solo *entonces* el agente lo procesa — ya sea inmediatamente o después de un retraso configurado (ver [Deferred Triggers](#trigger-deferred-delay)). Nada de lo que haga el agente añade latencia a la experiencia del usuario.

### Por qué utilizarlos

- **Moderar a gran escala.** Marcar spam evidente y prohibir a infractores reincidentes sin vigilar la cola todo el tiempo.
- **Dar la bienvenida a nuevos comentaristas.** Responder a los comentaristas primerizos con su propia voz.
- **Destacar el mejor contenido.** Fijar comentarios sustantivos de nivel superior una vez que superan un umbral de votos.
- **Aplicar sus directrices de forma consistente.** Aplicar el mismo texto de política a cada comentario borderline.
- **Resumir hilos extensos.** Publicar resúmenes neutrales de debates de varias páginas.

### Qué le mantiene bajo control

- **Modo de ejecución en seco.** Cada agente nuevo se entrega en **Dry Run**: procesa los disparadores, ejecuta el modelo y registra lo que *haría*, pero no realiza acciones reales. Ver [Dry-Run Mode](#dry-run-mode).
- **Aprobaciones.** Cualquier subconjunto de acciones puede estar condicionado a la aprobación humana. Ver [Approval Workflow](#approval-workflow).
- **Presupuestos por agente y por cuenta.** Límites duros diarios y mensuales. Ver [Budgets Overview](#budgets-overview).
- **Lista blanca de herramientas.** Las herramientas no permitidas se eliminan de la paleta del modelo — el agente literalmente no puede solicitarlas. Ver [Allowed Tool Calls Overview](#tools-overview).
- **Campos de auditoría en cada acción.** El modelo debe incluir una justificación y una puntuación de confianza. Ambos aparecen en la línea de tiempo de ejecución y en cada aprobación. Ver [Run Detail View](#run-detail-view).
- **Artículo 17 del EU DSA.** En la región de la UE, los baneos totalmente automatizados están bloqueados. Ver [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Sin entrenamiento con sus datos.** FastComments utiliza proveedores que no entrenan con sus indicaciones o comentarios.

### Dónde encajan junto a la moderación humana

Los agentes y los moderadores humanos comparten la misma plataforma de comentarios: los agentes realizan acciones a través de los mismos canales (aprobar, spam, prohibir, insignia, fijar, bloquear, escribir) y esas acciones aparecen en los mismos [Comment Logs](/guide-moderation.html#comment-logs), la misma [Moderation Page](/guide-moderation.html#moderate-comments-page) y los mismos flujos de notificaciones. Los agentes y los humanos ven el trabajo del otro y pueden reaccionar mutuamente — las acciones de los moderadores son, a su vez, disparadores válidos para los agentes (ver [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) y sus variantes).

Si también necesita personas en la cola, FastComments ofrece [Human Moderation Services](/guide-moderation.html#human-moderation-services): nuestros moderadores trabajan junto a sus agentes, en su panel, bajo sus directrices.