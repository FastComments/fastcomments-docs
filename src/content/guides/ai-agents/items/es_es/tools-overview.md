Un agente tiene **herramientas** que son las acciones que puede ejecutar. El formulario de edición del agente tiene una sección **Allowed tool calls** donde marcas las herramientas que este agente puede usar, y una sección **Approvals** donde marcas las acciones que deberían requerir la aprobación humana antes de que entren en vigor.

Hay tres niveles para cualquier herramienta:

- **Disallowed** - el agente no puede verla ni usarla.
- **Allowed, no approval** - el agente la usa directamente. Registrado en el historial de ejecuciones.
- **Allowed, with approval** - la llamada del agente se pone en cola para revisión humana y solo se ejecuta cuando un humano la aprueba.

Las herramientas denegadas son silenciosas: el agente no puede solicitarlas y la plataforma las rechaza por completo. Las herramientas con aprobación siempre pasan por la [bandeja de aprobaciones](#approval-workflow).

### Registro de auditoría en cada acción

Cada acción que realiza el agente se registra con una breve justificación (1-2 frases que expliquen por qué) y una puntuación de confianza (0.0-1.0). Ambos aparecen en la [Vista de detalles de ejecución](#run-detail-view) y en cada [aprobación](#approval-workflow). La búsqueda en memoria es la única excepción de solo lectura: no se registra como una acción y siempre está disponible independientemente de la lista de permitidos.

### Referencia de herramientas

#### Publicar comentarios

Permite al agente publicar un comentario como él mismo. El comentario se muestra públicamente bajo el nombre visible del agente. Utilizado por agentes de bienvenida y de resumen. Reversible: cualquier moderador puede eliminar un mal comentario. Poner bajo aprobación si tu comunidad necesita que cada mensaje público sea revisado por un humano.

#### Editar un comentario

Permite al agente reescribir el texto de un comentario dentro del alcance. El texto original se conserva en el registro de auditoría del comentario. Reservar para casos concretos: redactar información personal que un usuario filtró, o enmendar la propia respuesta anterior del agente. No para reescribir opiniones o suavizar el tono. Ver [Edit comment](#tool-edit-comment) para la página completa.

#### Votar en comentarios

Permite al agente votar a favor o en contra de un comentario. El voto cuenta para el total como cualquier otro voto. La mayoría de las comunidades prefieren que los bots no voten; no está habilitado en ninguna plantilla inicial. Si lo permites, el voto es reversible.

#### Fijar / desafijar un comentario

Permite al agente fijar un comentario en la parte superior de la página o desafijar uno que ya esté fijado. La plataforma no aplica una regla de un sólo pin por hilo, por lo que un agente que fije debe ser instruido para desafijar primero el comentario previamente fijado. Para descubrir qué ya está fijado en la misma página, el agente puede llamar a la herramienta de solo lectura `get_pinned_comments` (ver más abajo). Usado por la plantilla Top Comment Pinner.

#### Bloquear / desbloquear un comentario

Permite al agente impedir más respuestas bajo un comentario, o restaurar las respuestas. El comentario bloqueado sigue siendo visible. Útil para enfriar hilos acalorados, emparejado con un desbloqueo diferido. Para descubrir qué está bloqueado actualmente en la misma página, el agente puede llamar a la herramienta de solo lectura `get_locked_comments` (ver más abajo).

#### Marcar / desmarcar como spam

Permite al agente marcar un comentario como spam (ocultándolo a los lectores y alimentando al clasificador de spam) o limpiar esa bandera. La herramienta básica para cualquier agente de moderación. Reversible.

#### Aprobar / desaprobar un comentario

Permite al agente mostrar un comentario retenido a los lectores, u ocultar uno ya visible. Más útil en tenants que retienen nuevos comentarios para revisión de moderadores.

#### Marcar un comentario como revisado

Una herramienta de estado de cola: marca un comentario como "un moderador (o agente) ha visto esto". No cambia la visibilidad. Bajo riesgo; raramente requiere aprobación.

#### Otorgar una insignia

Permite al agente dar a un usuario una insignia que hayas configurado para tu tenant. Reversible por un moderador. Cuando esta herramienta está habilitada, el agente puede ver las insignias de tu tenant y elegir la adecuada por sí mismo, por lo que no necesitas pegar identificadores de insignias en las directrices de la comunidad o en el prompt inicial. Para dirigir qué insignia se otorga por qué comportamiento, referencia las insignias por su **Display Label** en el prompt.

#### Enviar correo electrónico

Permite al agente enviar un correo de texto plano al autor de un comentario dentro del alcance del disparador. El agente nunca ve la dirección de correo del destinatario: selecciona un comentario y la plataforma entrega al correo que ese comentarista dejó al publicar. La dirección remitente es el remitente con la marca de tu tenant (con DKIM) cuando el dominio del comentario coincide con un dominio configurado; de lo contrario, se usa el predeterminado de la plataforma. Usar con moderación: el correo electrónico es la herramienta de mayor fricción y los correos malos son difíciles de deshacer.

#### Guardar / buscar en la memoria del agente

Dos herramientas emparejadas que leen y escriben en un conjunto compartido de notas sobre el usuario para el que se activó un disparador. La memoria se comparte entre todos los agentes en tu tenant, por lo que las notas de un agente de triaje informan las decisiones de un agente moderador. La búsqueda es de solo lectura y siempre está disponible; guardar rara vez requiere aprobación. Ver [Agent Memory System](#agent-memory-system) para el diseño completo.

#### Obtener comentarios fijados / Obtener comentarios bloqueados

Dos herramientas de descubrimiento de solo lectura que listan los comentarios fijados (o bloqueados) en la misma página (`urlId`) en la que se activó el disparador. No toman argumentos: la página se lee desde el contexto del disparador, por lo que el agente no puede pivotar a otras páginas. Úsalas cuando un agente necesite actuar sobre un comentario que ya está fijado o bloqueado: típicamente la primera llamada antes de `unpin_comment` o `unlock_comment`, o antes de fijar un nuevo comentario para poder desafijar el existente primero.

Cada herramienta se habilita por separado en **Allowed tool calls** (el administrador marca `List pinned comments on the current page` o `List locked comments on the current page`). No pueden estar sujetas a aprobación: las herramientas de solo lectura no tienen efectos secundarios que aprobar. Llamarlas no se registra como una acción en el historial de ejecuciones; solo la llamada resultante `unpin_comment` / `unlock_comment` / `pin_comment` (si la hay) aparece. La lista está limitada a los 20 coincidencias más recientes por llamada.

Importante de entender: cuando una de estas herramientas devuelve un commentId, ese commentId se añade al alcance por ejecución del agente, por lo que la llamada subsiguiente `unpin_comment` / `unlock_comment` valida frente a la comprobación de seguridad de objetivo de herramienta de la plataforma. Sin llamar primero a la herramienta de descubrimiento, el agente no puede actuar sobre comentarios que no estén ya en el alcance inmediato del disparador. Así que un agente de tipo unpin normalmente tiene ambas herramientas habilitadas (por ejemplo, `get_pinned_comments` más `unpin_comment`).

#### Advertir a un usuario

Envía un DM privado advirtiendo a un usuario sobre un comentario específico, y registra atómicamente la advertencia en la memoria del agente. La política de escalado de la plataforma se construye en torno a esta herramienta: advertir primero, banear sólo si el usuario reincide. Ver [Warn user](#tool-warn-user) para la página completa.

#### Banear a un usuario

La herramienta más trascendental que puede llamar un agente. Banea a un usuario por una duración fija, opcionalmente como shadow ban, opcionalmente también baneando la IP, opcionalmente también eliminando todos los comentarios del usuario. Las dos opciones destructivas (IP, eliminar-todo) están protegidas por opt-ins adicionales en el formulario de edición. En la región de la UE, todos los baneos requieren aprobación humana (ver [Cumplimiento del artículo 17 de la DSA de la UE](#eu-dsa-compliance)). Ver [Ban user](#tool-ban-user) para la página completa.

### Subopciones de la herramienta de baneo

La herramienta Ban expone dos opciones destructivas - delete-all-comments y ban-by-IP - que están ocultas al modelo por completo hasta que las habilitas mediante la sección **Ban options** en el formulario de edición. Incluso si el modelo alucina el parámetro, la plataforma rechaza valores que no hayas habilitado. Ver [Ban user](#tool-ban-user).