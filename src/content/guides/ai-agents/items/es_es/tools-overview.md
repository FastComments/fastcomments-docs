Las **herramientas** de un agente son las acciones que puede realizar. El formulario de edición del agente tiene una sección **Llamadas de herramientas permitidas** donde marcas las herramientas que este agente puede usar, y una sección **Aprobaciones** donde marcas las acciones que deben requerir la aprobación humana antes de que entren en vigor.

There are three levels for any tool:

- **No permitido** - el agente no puede verlo ni usarlo.
- **Permitido, sin aprobación** - el agente lo usa directamente. Registrado en el historial de ejecución.
- **Permitido, con aprobación** - la llamada del agente se pone en cola para revisión humana y solo se ejecuta cuando un humano la aprueba.

Las herramientas no permitidas son silenciosas: el agente no puede solicitarlas y la plataforma las rechaza de plano. Las herramientas con aprobación siempre pasan por la [bandeja de aprobaciones](#approval-workflow).

### Registro de auditoría en cada acción

Cada acción que realiza el agente se registra con una breve justificación (1–2 frases que expliquen por qué) y una puntuación de confianza (0.0–1.0). Ambos aparecen en la [Vista de detalles de ejecución](#run-detail-view) y en cada [aprobación](#approval-workflow). La búsqueda en la memoria es la única excepción de solo lectura: no se registra como una acción y siempre está disponible independientemente de la lista de permitidos.

### Referencia de herramientas

#### Publicar comentarios

Permite al agente publicar un comentario como él mismo. El comentario se muestra públicamente bajo el nombre para mostrar del agente. Usado por agentes de bienvenida y de resumen. Reversible - cualquier moderador puede eliminar un comentario inapropiado. Normalmente permitido sin aprobación; actívalo con aprobación si tu comunidad necesita que cada mensaje público sea revisado por un humano.

#### Editar un comentario

Permite al agente reescribir el texto de un comentario dentro del alcance. El texto original se conserva en el registro de auditoría del comentario. Reservar para casos limitados - redactar información personal identificable (PII) que un usuario haya filtrado, o enmendar la propia respuesta previa del agente. No para reescribir opiniones o suavizar el tono. **Considere seriamente exigir aprobación.** Véase [Editar comentario](#tool-edit-comment) para la página completa.

#### Votar en comentarios

Permite al agente votar a favor o en contra de un comentario. El voto cuenta para el total de votos del comentario como cualquier otro voto. La mayoría de las comunidades prefieren que los bots no voten; no está habilitado en ninguna plantilla inicial. Si lo permites, el voto es reversible.

#### Anclar / desanclar un comentario

Permite al agente anclar un comentario en la parte superior de la página o desanclar uno que ya esté anclado. La plataforma no aplica una regla de un anclado por hilo, por lo que se debe instruir al agente que ancla para que primero desancle el comentario previamente anclado. Usado por la Top Comment Pinner template. Reversible; normalmente permitido sin aprobación.

#### Bloquear / desbloquear un comentario

Permite al agente impedir más respuestas bajo un comentario, o restaurarlas. El comentario bloqueado permanece visible. Útil para períodos de enfriamiento en hilos acalorados, combinado con un desbloqueo diferido. Reversible pero visible para tu comunidad; considera exigir aprobación en comunidades con alto riesgo.

#### Marcar / desmarcar como spam

Permite al agente marcar un comentario como spam (ocultándolo a los lectores y alimentando el clasificador de spam) o quitar esa marca. La herramienta básica para cualquier agente de moderación. Reversible. Considera seriamente exigir aprobación durante las primeras semanas mientras construyes la confianza en el agente.

#### Aprobar / retirar aprobación de un comentario

Permite al agente mostrar un comentario retenido a los lectores, o ocultar uno ya visible. Más útil en tenants que retienen nuevos comentarios para la revisión de moderadores. Es de alto riesgo al retirar la aprobación de un comentario visible - considera exigir aprobación.

#### Marcar un comentario como revisado

Una herramienta de estado de la cola: marca un comentario como "un moderador (o agente) lo ha revisado". No cambia la visibilidad. Bajo riesgo; raramente requiere aprobación.

#### Otorgar una insignia

Permite al agente otorgar a un usuario una insignia de la configuración de insignias de tu tenant. Reversible por un moderador. Rara vez requiere aprobación. El agente debe conocer el ID de la insignia, así que incluye los IDs relevantes en tus [directrices de la comunidad](#community-guidelines) o en el [prompt inicial](#personality-prompt).

#### Enviar correo electrónico

Permite al agente enviar un correo electrónico en texto plano desde `noreply@fastcomments.com` a una dirección que elija. Úsalo con moderación - el correo electrónico es la herramienta de mayor fricción y los correos erróneos son difíciles de deshacer. Considera seriamente exigir aprobación, y dirige los correos de aprobación a quien posea la bandeja de entrada a la que el agente terminará enviando mensajes.

#### Guardar / buscar en la memoria del agente

Dos herramientas emparejadas que leen y escriben en un conjunto compartido de notas sobre el usuario para el que se activó un disparador. La memoria se comparte entre todos los agentes de tu tenant, por lo que las notas de un agente de triaje informan las decisiones de un agente moderador. La búsqueda es solo de lectura y siempre está disponible; guardar rara vez requiere aprobación. Véase el [Sistema de memoria de agentes](#agent-memory-system) para el diseño completo.

#### Advertir a un usuario

Envía un aviso privado por DM a un usuario sobre un comentario específico, y registra de forma atómica la advertencia en la memoria del agente. La política de escalamiento de la plataforma se basa en esta herramienta - advertir primero, banear solo si el usuario reincide. Menos frecuentemente requiere aprobación que `ban_user`, pero considera exigir aprobación durante las primeras semanas de vida del agente. Véase [Avisar a un usuario](#tool-warn-user) para la página completa.

#### Bloquear a un usuario

La herramienta más trascendental que puede invocar un agente. Bloquea a un usuario por una duración fija, opcionalmente como shadow ban, opcionalmente también bloqueando la IP, opcionalmente también eliminando todos los comentarios del usuario. Las dos opciones destructivas (IP, delete-all) están sujetas a opt-ins adicionales en el formulario de edición. En la región de la UE, todos los bloqueos requieren aprobación humana (véase [Cumplimiento del artículo 17 de la DSA de la UE](#eu-dsa-compliance)). Considera seriamente exigir aprobación en todas partes. Véase [Bloquear a un usuario](#tool-ban-user) para la página completa.

### Subopciones de la herramienta de baneo

La herramienta de baneo expone dos opciones destructivas - delete-all-comments y ban-by-IP - que están ocultas al modelo por completo hasta que las habilites mediante la sección **Opciones de baneo** en el formulario de edición. Incluso si el modelo alucina el parámetro, la plataforma rechaza valores en los que no te hayas inscrito. Véase [Bloquear a un usuario](#tool-ban-user).