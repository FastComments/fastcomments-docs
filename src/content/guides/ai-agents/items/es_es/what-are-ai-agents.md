Un **AI Agent** es un trabajador autónomo, limitado a tu tenant de FastComments, que vigila los eventos en tu comunidad y actúa en tu nombre.

Cada agente tiene tres cosas que controlas:

1. **Una personalidad.** Un prompt de texto libre inicial que define el tono, el rol y el estilo de toma de decisiones ("Eres un cálido recepcionista de la comunidad", "Haces cumplir las normas comunitarias pero tiendes a advertir antes que prohibir", etc.).
2. **Uno o más activadores.** Una lista de eventos que despiertan al agente: un nuevo comentario, un comentario que supera un umbral de votos o reportes, una acción de moderador, el primer comentario de un usuario en el sitio, entre otros. La lista completa está en [Trigger Events Overview](#triggers-overview).
3. **Una lista de herramientas permitidas.** Lo que el agente puede hacer: publicar un comentario, votar, fijar, bloquear, marcar como spam, prohibir a un usuario, advertir vía DM, otorgar una insignia, enviar correo, guardar y buscar en una memoria compartida. La lista completa está en [Allowed Tool Calls Overview](#tools-overview).

Cuando se activa un activador, el agente recibe un mensaje de contexto que describe lo sucedido (el comentario, la página, contexto opcional del hilo/usuario/página) y se le proporciona su prompt inicial y las normas de tu comunidad. Entonces invoca herramientas para actuar, registrando una justificación y una puntuación de confianza con cada llamada.

### Los agentes se ejecutan de forma asincrónica

Los agentes **nunca bloquean la acción del usuario que los activó**. Un lector publica un comentario, el comentario se guarda y se muestra en el hilo, se devuelve la respuesta, y solo *después* se ejecuta el agente sobre él — ya sea inmediatamente o tras un retraso configurado (ver [Deferred Triggers](#trigger-deferred-delay)). Nada de lo que haga el agente añade latencia a la experiencia visible para el usuario.

### Por qué usarlos

- **Modera a escala.** Marca el spam obvio y prohíbe a reincidentes sin vigilar la cola las 24 horas.
- **Da la bienvenida a nuevos comentaristas.** Responde a comentaristas primerizos con tu voz.
- **Destaca el mejor contenido.** Fija comentarios sustantivos de nivel superior una vez superen un umbral de votos.
- **Aplica tus normas de forma coherente.** Aplica el mismo texto de política a cada comentario en el límite.
- **Resume hilos extensos.** Publica resúmenes neutrales de debates de varias páginas.

### Qué te mantiene en control

- **Modo Dry Run.** Todo nuevo agente se entrega en **Dry Run**: procesa activadores, ejecuta el modelo y registra lo que *haría*, pero no realiza acciones reales. Ver [Dry-Run Mode](#dry-run-mode).
- **Aprobaciones.** Cualquier subconjunto de acciones puede requerir aprobación humana. Ver [Approval Workflow](#approval-workflow).
- **Presupuestos por agente y por cuenta.** Límites estrictos diarios y mensuales. Ver [Budgets Overview](#budgets-overview).
- **Lista de herramientas permitidas.** Las herramientas no permitidas se eliminan de la paleta del modelo: el agente literalmente no puede solicitarlas. Ver [Allowed Tool Calls Overview](#tools-overview).
- **Campos de auditoría en cada acción.** El modelo debe incluir una justificación y una puntuación de confianza. Ambos aparecen en la línea de tiempo de la ejecución y en cada aprobación. Ver [Run Detail View](#run-detail-view).
- **EU DSA Article 17.** En la región de la UE, las prohibiciones totalmente automatizadas están bloqueadas. Ver [Cumplimiento del Artículo 17 de la DSA de la UE](#eu-dsa-compliance).
- **No se entrena con tus datos.** FastComments utiliza proveedores que no entrenan con tus prompts o comentarios.

### Dónde encajan junto a la moderación humana

Los agentes y los moderadores humanos comparten la misma plataforma de comentarios: los agentes realizan acciones a través de los mismos canales (approve, spam, ban, badge, pin, lock, write) y esas acciones aparecen en los mismos [Registros de comentarios](/guide-moderation.html#comment-logs), la misma [Página de moderación](/guide-moderation.html#moderate-comments-page) y los mismos flujos de notificaciones. Los agentes y los humanos ven el trabajo del otro y pueden reaccionar entre sí: las acciones de los moderadores son en sí mismas activadores válidos para agentes (ver [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) y similares).