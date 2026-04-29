Las **herramientas** de un agente son las acciones que puede realizar. El formulario de edición del agente tiene una sección **Allowed tool calls** donde marcas las herramientas que este agente puede usar, y una sección **Approvals** donde marcas las acciones que deben requerir la aprobación de una persona antes de que surtan efecto.

Hay tres niveles para cualquier herramienta:

- **Disallowed** - el agente no puede verla ni usarla.
- **Allowed, no approval** - el agente la usa directamente. Registrado en el historial de ejecución.
- **Allowed, with approval** - la llamada del agente se pone en cola para revisión humana y solo se ejecuta cuando una persona la aprueba.

Las herramientas no permitidas son silenciosas: el agente no puede solicitarlas y la plataforma las rechaza por completo. Las herramientas que requieren aprobación siempre pasan por la [bandeja de entrada de aprobaciones](#approval-workflow).

### Registro de auditoría en cada acción

Cada acción que realiza el agente se registra con una breve justificación (1-2 frases que expliquen por qué) y una puntuación de confianza (0.0-1.0). Ambos aparecen en la [Vista de detalle de ejecución](#run-detail-view) y en cada [aprobación](#approval-workflow). La búsqueda en la memoria es la única excepción de solo lectura: no se registra como una acción y siempre está disponible independientemente de la lista de permitidos.

### Referencia de herramientas

#### Publicar comentarios

Permite que el agente publique un comentario como él mismo. El comentario se muestra públicamente bajo el nombre visible del agente. Usado por agentes saludadores y resumidores. Reversible: cualquier moderador puede eliminar un comentario inapropiado. Normalmente permitido sin aprobación; pónselo como restricción si tu comunidad necesita que cada mensaje público sea revisado por una persona.

#### Votar en comentarios

Permite que el agente vote a favor o en contra de un comentario. El voto cuenta para el total de votos del comentario como cualquier otro voto. La mayoría de las comunidades prefieren que los bots no voten; no está habilitado en ninguna plantilla inicial. Si lo permites, el voto es reversible.

#### Anclar / desanclar un comentario

Permite que el agente ancle un comentario en la parte superior de la página o desancle uno que ya está anclado. La plataforma no hace cumplir una regla de un anclaje por hilo, por lo que a un agente que ancle se le debe indicar que primero desancle el comentario previamente anclado. Usado por la plantilla Top Comment Pinner. Reversible; normalmente permitido sin aprobación.

#### Bloquear / desbloquear un comentario

Permite que el agente impida más respuestas bajo un comentario, o restaure las respuestas. El comentario bloqueado sigue siendo visible. Útil para enfriar hilos acalorados, combinado con un desbloqueo diferido. Reversible pero visible para tu comunidad; considera ponerlo bajo aprobación en comunidades con mucho en juego.

#### Marcar / desmarcar como spam

Permite que el agente marque un comentario como spam (ocultándolo a los lectores y alimentando al clasificador de spam) o elimine esa marca. La herramienta básica para cualquier agente de moderación. Reversible. Considera seriamente ponerla bajo aprobación durante las primeras semanas mientras construyes confianza en el agente.

#### Aprobar / desaprobar un comentario

Permite que el agente muestre a los lectores un comentario retenido, u oculte uno que ya esté visible. Muy útil en tenants que retienen comentarios nuevos para revisión de moderadores. Alta responsabilidad cuando se desaprueba un comentario visible; considera ponerlo bajo aprobación.

#### Marcar un comentario como revisado

Herramienta de estado de cola: marca un comentario como "un moderador (o agente) ha revisado esto". No cambia la visibilidad. Bajo riesgo; rara vez se restringe.

#### Otorgar una insignia

Permite que el agente entregue a un usuario una insignia de la configuración de insignias de tu tenant. Reversible por un moderador. Rara vez restringida. El agente debe conocer el ID de la insignia, así que incluye los IDs relevantes en tus [directrices de la comunidad](#community-guidelines) o en el [prompt inicial](#personality-prompt).

#### Enviar correo electrónico

Permite que el agente envíe un correo de texto plano desde `noreply@fastcomments.com` a una dirección que elija. Úsalo con moderación: el correo electrónico es la herramienta de más fricción y los correos dañinos son difíciles de deshacer. Considera seriamente ponerlo bajo aprobación y dirigir los correos de aprobación a quien maneje la bandeja de entrada a la que el agente acabará enviando mensajes.

#### Guardar / buscar memoria del agente

Dos herramientas emparejadas que leen y escriben en un conjunto compartido de notas sobre el usuario para el que se activó un disparador. La memoria se comparte entre todos los agentes en tu tenant, por lo que las notas de un agente de triaje informan las decisiones de un agente moderador. La búsqueda es de solo lectura y siempre está disponible; el guardado rara vez se restringe. Consulta el [Sistema de memoria de agentes](#agent-memory-system) para el diseño completo.

#### Advertir a un usuario

Envía un DM privado de advertencia a un usuario sobre un comentario específico y registra de forma atómica la advertencia en la memoria del agente. La política de escalamiento de la plataforma se basa en esta herramienta: advertir primero, banear solo si el usuario reincide. Menos frecuentemente restringida que `ban_user`, pero considera restringirla durante las primeras semanas de vida de un agente. Consulta [Advertir al usuario](#tool-warn-user) para la página completa.

#### Banear a un usuario

La herramienta con mayores consecuencias que puede invocar un agente. Banea a un usuario por una duración fija, opcionalmente como shadow ban, opcionalmente también baneando la IP, opcionalmente también eliminando todos los comentarios del usuario. Las dos opciones destructivas (IP, eliminar todos) están protegidas detrás de opt-ins adicionales en el formulario de edición. En la región de la UE, todos los baneos requieren aprobación humana (ver [Cumplimiento del Artículo 17 de la DSA de la UE](#eu-dsa-compliance)). Considera seriamente ponerla bajo aprobación en todas partes. Consulta [Banear a un usuario](#tool-ban-user) para la página completa.

### Sub-opciones de la herramienta de baneo

La herramienta Ban expone dos opciones destructivas - delete-all-comments y ban-by-IP - que están ocultas para el modelo por completo hasta que las habilites mediante la sección **Ban options** en el formulario de edición. Incluso si el modelo alucina el parámetro, la plataforma rechaza los valores en los que no te inscribiste. Consulta [Banear a un usuario](#tool-ban-user).