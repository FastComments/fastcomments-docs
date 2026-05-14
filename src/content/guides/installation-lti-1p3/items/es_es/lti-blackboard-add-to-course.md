Una vez que un administrador haya registrado FastComments como una herramienta LTI 1.3 Advantage y aprobado las políticas de la institución, los docentes la añaden a los cursos a través de los puntos de colocación estándar de Blackboard. Los pasos exactos difieren entre Ultra Course View y Original Course View, así que ambos se cubren a continuación.

#### Ultra Course View

Ultra Course View es la vista predeterminada en Blackboard Learn SaaS a partir de 2026.

1. Abra el curso y vaya a la página **Course Content**.
2. Pase el cursor o toque donde quiera que aparezca el hilo de comentarios en el esquema y haga clic en el botón morado **+** (Agregar contenido).
3. Elija **Content Market**. El panel Content Market muestra todas las herramientas LTI y ubicaciones de Building Block aprobadas para su institución.
4. Busque el mosaico **FastComments** y haga clic en él. Blackboard crea un elemento de contenido en la posición donde abrió el menú **+**.
5. El elemento aparece en el esquema como una entrada "Visible to students" por defecto para los docentes que tienen **Hide from students** desactivado como su valor predeterminado personal. Si su valor predeterminado es **Hidden**, el elemento se crea oculto y activa el selector de visibilidad en la fila del elemento cuando esté listo.
6. Para cambiar el nombre del elemento, haga clic en el título en el esquema y escriba una nueva etiqueta. El título que ven los estudiantes en el esquema es independiente del identificador del hilo de FastComments, por lo que renombrar es seguro en cualquier momento.

Si no ve **Content Market** como opción, su institución tiene la colocación oculta. También puede acceder al mismo selector a través de **More tools** en el mismo menú **+** bajo el grupo **LTI Tools**.

#### Original Course View

Original Course View todavía es compatible en Learn SaaS y sigue siendo la experiencia principal para sitios Learn 9.1 autohospedados en la línea de lanzamiento CU Q4 2024.

1. Abra el curso y entre en un **Content Area** (por ejemplo, el área por defecto **Information** o **Content** en el menú del curso).
2. Active **Edit Mode** con el interruptor en la esquina superior derecha de la página.
3. Haga clic en **Build Content** en la barra de acciones.
4. Bajo el submenú **Learning Tools**, haga clic en **FastComments**. El submenú Learning Tools se rellena a partir de las ubicaciones de herramientas LTI 1.3 después de que un administrador registre la herramienta. Si no lo ve, consulte la sección de problemas comunes más abajo.
5. En el formulario **Create FastComments**, configure:
   - **Name**: la etiqueta que los estudiantes ven en el área de contenido.
   - **Description**: texto opcional que se muestra encima del hilo incrustado.
   - **Permit Users to View this Content**: conmutador de disponibilidad Sí/No.
   - **Track Number of Views**: habilítelo si desea las estadísticas de vistas por elemento de Blackboard. FastComments ejecuta sus propias analíticas de forma independiente.
   - **Date and Time Restrictions**: ventanas opcionales **Display After** / **Display Until**.
6. Envíe el formulario. La herramienta aparece como un elemento clicable en el área de contenido.

#### Incrustación dentro de un elemento o documento

En ambas vistas del curso, los docentes incrustan FastComments en línea dentro del cuerpo de un Item, Document o cualquier campo de texto enriquecido mediante el botón LTI Advantage del Editor de Contenido.

Ultra Course View:

1. Cree o edite un **Document**.
2. Haga clic en **Add content** dentro del cuerpo del documento donde quiera que aparezca el hilo.
3. En la barra de herramientas del editor, abra el menú **Insert content** y haga clic en **Content Market** (el punto de entrada LTI Advantage / Deep Linking).
4. Elija **FastComments**. FastComments devuelve una carga útil de deep-link y Blackboard inserta un bloque incrustado en el cuerpo del documento en la posición del cursor.
5. Guarde el documento. Los estudiantes ven el hilo representado en línea a medida que se desplazan por él.

Original Course View:

1. Edite cualquier elemento con un cuerpo de texto enriquecido.
2. En la barra de herramientas del Editor de Contenido, haga clic en el icono de más **Add Content** y elija **Content Market** (etiquetado como **Add Content from External Tool** en CUs antiguas de Q4 2024).
3. Elija **FastComments**. El editor inserta un bloque marcador de posición que referencia el recurso deep-linked.
4. Envíe el elemento.

Cada inserción deep-link crea su propio hilo de FastComments, por lo que un Item con dos bloques FastComments incrustados tiene dos flujos de comentarios independientes.

#### Visibilidad, condiciones de publicación y restricciones de grupo

Los elementos de contenido de FastComments se comportan como cualquier otro elemento de contenido de Blackboard respecto a las reglas de control de acceso que se aplican sobre ellos.

- Ultra: haga clic en el selector de visibilidad en la fila (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilidad condicional admite ventanas de fecha/hora, reglas de rendimiento contra elementos del libro de calificaciones y reglas de miembros contra grupos del curso.
- Original: abra el menú contextual del elemento y elija **Adaptive Release** o **Adaptive Release: Advanced** para controlar el acceso a la herramienta por fecha, membresía, calificación o estado de revisión. Use **Set Group Availability** en el elemento para restringirlo a grupos específicos del curso.

FastComments respeta lo que decida la puerta de Blackboard. Si Blackboard oculta el elemento a un estudiante, el lanzamiento LTI nunca ocurre para ese estudiante y este no aparece en la vista de moderador.

#### Comportamiento en el libro de calificaciones

FastComments no informa calificaciones a través de LTI Advantage Assignment and Grade Services. No se crea automáticamente una columna de calificaciones para los elementos de contenido de FastComments.

Si su tenant de Blackboard está configurado para crear automáticamente una columna del libro de calificaciones para cada nuevo elemento de contenido independientemente de los metadatos de evaluación, aparecerá igualmente una columna vacía. Para ocultarla:

- Ultra: abra el **Gradebook**, haga clic en el encabezado de la columna, elija **Edit** y apague **Show to students** junto con **Include in calculations**. O use **Delete** si su institución permite eliminar columnas para elementos no evaluados.
- Original: abra el **Grade Center**, haga clic en el chevrón de la columna, elija **Hide from Users (on/off)** y, opcionalmente, **Hide from Instructor View** en **Column Organization**.

#### Lo que ven los estudiantes

Cuando un estudiante abre el elemento de FastComments o se desplaza hasta un bloque incrustado:

1. Blackboard lanza el mensaje LTI 1.3 a FastComments. El estudiante inicia sesión mediante SSO usando su identidad de Blackboard (nombre, correo electrónico, avatar, rol) sin ver un formulario de inicio de sesión.
2. El hilo de comentarios se renderiza en el iframe. El anidamiento, las respuestas, las menciones y las reacciones están disponibles según la configuración del widget de comentarios configurada en FastComments.
3. Sus comentarios se atribuyen a su cuenta de Blackboard. Si el estudiante edita su nombre o foto en Blackboard más adelante, el siguiente lanzamiento actualiza el perfil de FastComments.

Mapeo de roles de Blackboard a FastComments:

- **System Administrator** y **Course Builder** se mapean a **admin** en FastComments.
- **Instructor** y **Teaching Assistant** se mapean a **moderator** en FastComments.
- **Student**, **Guest** y **Observer** se mapean a **commenter** en FastComments.

Los moderadores ven los controles de moderación (fijar, ocultar, banear, eliminar) en línea en cada comentario del hilo.

#### Alcance de los hilos

FastComments delimita cada hilo por **(Blackboard host, course ID, resource link ID)**. Dos elementos de FastComments en el mismo curso producen dos hilos. El mismo elemento copiado en dos entornos de curso (por ejemplo, mediante copia de curso) produce dos hilos, porque Blackboard emite un nuevo resource link ID durante la copia. Para mantener un hilo compartido entre copias de curso, use Deep Linking con un URN de hilo explícito configurado en FastComments antes de lanzar la copia.

#### Problemas específicos de Blackboard

**Mosaico de FastComments ausente en el menú Build Content (Original) o en Content Market (Ultra).** El administrador aprobó la herramienta pero dejó una política de la institución que bloquea la colocación relevante. Vaya a **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edite la entrada de FastComments y confirme que las colocaciones **Course Content Tool** (Original) y **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) están habilitadas. Guarde y actualice la página del curso.

**Error "Tool not configured for this context" o "Tool is not deployed" al iniciar.** El ámbito de implementación registrado durante el registro dinámico no coincide con el contexto de la institución al que pertenece el curso. En la entrada del proveedor de herramientas de Blackboard, verifique que el **Deployment ID** coincida con lo que FastComments muestra en su página de configuración LTI 1.3 para este tenant. Si difieren, elimine la colocación y vuelva a ejecutar el registro dinámico desde una URL de registro nueva.

**La altura del iframe parece fija o el contenido se corta.** Algunos tenants de Blackboard incluyen una política de seguridad de contenido (Content Security Policy) estricta que bloquea el postMessage de cambio de tamaño de iframe LTI por defecto. FastComments emite tanto el mensaje estilo Canvas `lti.frameResize` como el mensaje del estándar IMS `org.imsglobal.lti.frameResize` para maximizar la compatibilidad, pero una anulación de CSP a nivel tenant puede bloquear el listener del padre. Pida a su administrador que confirme que `*.fastcomments.com` está en la lista blanca de herramientas LTI y que ningún encabezado CSP personalizado está eliminando los eventos postMessage. Entonces el cambio de tamaño funcionará sin configuración adicional.

**La copia de curso duplica hilos.** La copia de curso de Blackboard emite nuevos resource link IDs para las colocaciones LTI, por lo que los cursos copiados comienzan con hilos vacíos. Esto es esperado. Si necesita que el curso copiado herede el hilo original, configure Deep Linking con un URN de hilo explícito antes de copiar, o contacte al soporte de FastComments para reasignar IDs de hilo en bloque.

**El estudiante ve un error genérico de Blackboard al iniciar.** La causa es un claim `email` faltante o desactualizado. Confirme que la política de la institución para FastComments tiene habilitados **Role**, **Name**, y **Email Address** bajo **User Fields to Send**. Guarde y luego vuelva a iniciar en una sesión de navegador nueva.