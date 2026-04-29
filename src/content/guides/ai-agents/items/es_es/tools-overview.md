Las **herramientas** de un agente son las acciones que puede realizar. El formulario de edición del agente tiene una sección **Llamadas de herramienta permitidas** donde marcas las herramientas que este agente puede usar, y una sección **Aprobaciones** donde marcas las acciones que deben requerir la aprobación humana antes de que surtan efecto.

Hay tres niveles para cualquier herramienta:

- **No permitida** - el agente no puede verla ni usarla.
- **Permitida, sin aprobación** - el agente la usa directamente. Registrado en el historial de ejecución.
- **Permitida, con aprobación** - la llamada del agente se encola para revisión humana y solo se ejecuta cuando un humano la aprueba.

Las herramientas no permitidas son silenciosas: el agente no puede solicitarlas y la plataforma las rechaza de plano. Las herramientas sujetas a aprobación siempre pasan por la [bandeja de aprobaciones](#approval-workflow).

### Registro de auditoría en cada acción

Cada acción que realiza el agente se registra con una breve justificación (1-2 frases explicando por qué) y una puntuación de confianza (0.0-1.0). Ambos aparecen en la [Vista de detalle de ejecución](#run-detail-view) y en cada [aprobación](#approval-workflow). La búsqueda en la memoria es la única excepción de solo lectura: no se registra como una acción y siempre está disponible independientemente de la lista permitida.

### Referencia de herramientas

#### Publicar comentarios

Permite al agente publicar un comentario como sí mismo. El comentario se muestra públicamente bajo el nombre visible del agente. Usado por agentes saludadores y resumidores. Reversible: cualquier moderador puede eliminar un comentario inapropiado. Generalmente permitido sin aprobación; resérvalo si tu comunidad necesita que cada mensaje público sea revisado por un humano.

#### Editar un comentario

Permite al agente reescribir el texto de un comentario dentro del alcance. El texto original se conserva en el registro de auditoría del comentario. Reservar para casos reducidos: redactar información personal identificable que un usuario filtró, o enmendar la propia respuesta anterior del agente. No para reescribir opiniones o suavizar el tono. **Considere seriamente enmarcarlo detrás de una aprobación.** Véase [Editar comentario](#tool-edit-comment) para la página completa.

#### Votar en comentarios

Permite al agente votar a favor o en contra de un comentario. El voto cuenta para el total de votos del comentario como cualquier otro voto. La mayoría de las comunidades prefieren que los bots no voten; no está habilitado en ninguna plantilla inicial. Si lo permites, el voto es reversible.

#### Fijar / desafijar un comentario

Permite al agente fijar un comentario en la parte superior de la página o desafijar uno que ya esté fijado. La plataforma no aplica una regla de una sola fijación por hilo, por lo que a un agente que fije se le debe indicar que desafije el comentario previamente fijado primero. Usado por la plantilla Top Comment Pinner. Reversible; generalmente permitido sin aprobación.

#### Bloquear / desbloquear un comentario

Permite al agente impedir más respuestas bajo un comentario, o restaurar las respuestas. El comentario bloqueado sigue siendo visible. Útil para enfriar hilos acalorados, en combinación con un desbloqueo diferido. Reversible pero visible para tu comunidad; considera someterlo a aprobación en comunidades de alto riesgo.

#### Marcar / desmarcar como spam

Permite al agente marcar un comentario como spam (ocultándolo a los lectores y alimentando al clasificador de spam) o eliminar esa marca. La herramienta básica para cualquier agente de moderación. Reversible. Considera seriamente someterlo a aprobación durante las primeras semanas mientras construyes confianza en el agente.

#### Aprobar / desaprobar un comentario

Permite al agente mostrar un comentario en espera a los lectores, o ocultar uno ya visible. Más útil en inquilinos que retienen nuevos comentarios para revisión de moderadores. De alto riesgo cuando se desaprueba un comentario visible: considera someterlo a aprobación.

#### Marcar un comentario como revisado

Una herramienta de estado de cola: marca un comentario como "un moderador (o agente) ha mirado esto." No cambia la visibilidad. Bajo riesgo; rara vez sujeto a aprobación.

#### Otorgar una insignia

Permite al agente otorgar a un usuario una insignia que hayas configurado para tu inquilino. Reversible por un moderador. Rara vez sujeto a aprobación. Cuando esta herramienta está habilitada, el agente puede ver las insignias de tu inquilino y elegir la adecuada por sí mismo, por lo que no necesitas pegar identificadores de insignias en tus directrices comunitarias o en el aviso inicial. Si quieres orientar qué insignia se otorga por qué comportamiento, refiérete a las insignias por su **Etiqueta de visualización** en el aviso.

#### Enviar correo electrónico

Permite al agente enviar un correo electrónico en texto plano al autor de un comentario dentro del alcance del disparador. El agente nunca ve la dirección de correo del destinatario: elige un comentario y la plataforma lo entrega a la dirección que ese comentarista dejó cuando publicó. La dirección de remitente es la del emisor con marca de tu inquilino (con DKIM) cuando el dominio del comentario coincide con un dominio configurado, de lo contrario se usa la predeterminada de la plataforma. Úsalo con moderación: el correo electrónico es la herramienta de mayor fricción y los correos malos son difíciles de deshacer. Considera seriamente someterlo a aprobación, y dirige los correos de aprobación a quien sea dueño del buzón al que el agente terminará enviando correos.

#### Guardar / buscar memoria del agente

Dos herramientas emparejadas que leen y escriben en un pool compartido de notas sobre el usuario para el que se activó un disparador. La memoria se comparte entre todos los agentes de tu inquilino, por lo que las notas de un agente de triage informan las decisiones de un agente moderador. La búsqueda es de solo lectura y siempre está disponible; guardar rara vez se somete a aprobación. Véase [Sistema de memoria del agente](#agent-memory-system) para el diseño completo.

#### Advertir a un usuario

Envía un DM privado advirtiendo a un usuario sobre un comentario específico, y registra de forma atómica la advertencia en la memoria del agente. La política de escalamiento de la plataforma se construye alrededor de esta herramienta: advertir primero, prohibir solo si el usuario reincide. Menos comúnmente sujeta a aprobación que `ban_user`, pero considera someterla a aprobación durante las primeras semanas de vida de un agente. Véase [Advertir al usuario](#tool-warn-user) para la página completa.

#### Prohibir a un usuario

La herramienta más trascendental que un agente puede invocar. Prohíbe a un usuario por una duración fija, opcionalmente como prohibición en la sombra, opcionalmente también bloqueando la IP, opcionalmente también eliminando todos los comentarios del usuario. Las dos opciones destructivas (IP, eliminar-todo) están sujetas a opt-ins adicionales en el formulario de edición. En la región de la UE, todas las prohibiciones requieren aprobación humana (véase [Cumplimiento del Artículo 17 de la DSA de la UE](#eu-dsa-compliance)). Considera seriamente someterla a aprobación en todas partes. Véase [Prohibir usuario](#tool-ban-user) para la página completa.

### Sub-opciones de la herramienta de prohibición

La herramienta de Prohibición expone dos opciones destructivas - delete-all-comments y ban-by-IP - que están ocultas para el modelo por completo hasta que las habilites mediante la sección **Opciones de prohibición** en el formulario de edición. Incluso si el modelo alucina el parámetro, la plataforma rechaza los valores que no hayas optado por permitir. Véase [Prohibir usuario](#tool-ban-user).