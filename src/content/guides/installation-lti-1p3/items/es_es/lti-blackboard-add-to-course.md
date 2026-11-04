Una vez que un administrador haya registrado FastComments como herramienta LTI 1.3 Advantage y aprobado las políticas de la institución, los instructores lo añaden a los cursos a través de los puntos de colocación estándar de Blackboard. Los pasos exactos difieren entre Ultra Course View y Original Course View, por lo que ambos se cubren a continuación.

#### Ultra Course View

Ultra Course View es la opción predeterminada en Blackboard Learn SaaS a partir de 2026.

1. Abra el curso y vaya a la página **Course Content**.
2. Pase el cursor o toque donde desee que aparezca el hilo de comentarios en el esquema y haga clic en el botón morado **+** (Add content).
3. Elija **Content Market**. El panel de Content Market enumera todas las herramientas LTI aprobadas y las colocaciones de Building Block para su institución.
4. Busque el mosaico de **FastComments** y haga clic en él. Blackboard crea un elemento de contenido en la posición donde abrió el menú **+**.
5. El elemento aparece en el esquema como una entrada "Visible to students" de forma predeterminada para los instructores que tienen **Hide from students** desactivado como preferencia personal. Si su valor predeterminado es **Hidden**, el elemento se crea oculto y puede cambiar el selector de visibilidad en la fila del elemento cuando esté listo.
6. Para renombrar el elemento, haga clic en el título en el esquema y escriba una nueva etiqueta. El título que los estudiantes ven en el esquema es independiente del identificador del hilo de FastComments, por lo que renombrar es seguro en cualquier momento.

Si no ve **Content Market** como opción, su institución tiene oculta la colocación. También puede acceder al mismo selector a través de **More tools** en el mismo menú **+**, bajo el grupo **LTI Tools**.

#### Original Course View

Original Course View sigue siendo compatible en Learn SaaS y sigue siendo la experiencia principal para los sitios autohospedados de Learn 9.1 en la línea de versiones Q4 2024 CU.

1. Abra el curso y entre en un **Content Area** (por ejemplo, el área predeterminada **Information** o **Content** en el menú del curso).
2. Active **Edit Mode** con el interruptor en la parte superior derecha de la página.
3. Haga clic en **Build Content** en la barra de acciones.
4. Bajo el submenú **Learning Tools**, haga clic en **FastComments**. El submenú Learning Tools se completa a partir de las colocaciones de herramientas LTI 1.3 después de que un administrador registre la herramienta. Si no lo ve, consulte la sección de problemas conocidos a continuación.
5. En el formulario **Create FastComments**, configure:
   - **Name**: la etiqueta que los estudiantes ven en el área de contenido.
   - **Description**: texto opcional que se muestra encima del hilo incrustado.
   - **Permit Users to View this Content**: interruptor de disponibilidad Sí/No.
   - **Track Number of Views**: actívelo si desea las estadísticas de vistas por elemento de Blackboard. FastComments ejecuta su propio análisis de forma independiente.
   - **Date and Time Restrictions**: ventanas opcionales de **Display After** / **Display Until**.
6. Envíe. La herramienta aparece como un elemento en el que se puede hacer clic en el área de contenido.

#### Incrustación dentro de un elemento o documento

En ambas vistas de curso, los instructores incrustan FastComments en línea dentro del cuerpo de un Item, Document o cualquier campo de texto enriquecido a través del botón LTI Advantage del Content Editor.

Ultra Course View:

1. Cree o edite un **Document**.
2. Haga clic en **Add content** dentro del cuerpo del documento donde desee que aparezca el hilo.
3. En la barra de herramientas del editor, abra el menú **Insert content** y haga clic en **Content Market** (el punto de entrada de LTI Advantage / Deep Linking).
4. Elija **FastComments**. FastComments devuelve una carga útil de deep link y Blackboard inserta un bloque incrustado en el cuerpo del documento en la posición del cursor.
5. Guarde el documento. Los estudiantes ven el hilo renderizado en línea a medida que se desplazan por él.

Original Course View:

1. Edite cualquier elemento con un cuerpo de texto enriquecido.
2. En la barra de herramientas del Content Editor, haga clic en el icono más de **Add Content** y elija **Content Market** (etiquetado como **Add Content from External Tool** en versiones Q4 2024 CU más antiguas).
3. Elija **FastComments**. El editor inserta un bloque de marcador de posición que hace referencia al recurso de deep link.
4. Envíe el elemento.

Cada incrustación de deep link produce su propio hilo de FastComments, por lo que un Item con dos bloques de FastComments incrustados tiene dos flujos de comentarios independientes.

#### Visibilidad, condiciones de publicación y restricciones de grupo

Los elementos de contenido de FastComments se comportan como cualquier otro elemento de contenido de Blackboard para las reglas de control de acceso superpuestas sobre ellos.

- Ultra: haga clic en el selector de visibilidad en la fila (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilidad condicional admite ventanas de fecha/hora, reglas de rendimiento contra elementos del libro de calificaciones y reglas de miembros contra grupos del curso.
- Original: abra el menú contextual del elemento y elija **Adaptive Release** o **Adaptive Release: Advanced** para restringir la herramienta por fecha, pertenencia, calificación o estado de revisión. Use **Set Group Availability** en el elemento para restringirlo a grupos específicos del curso.

FastComments respeta lo que decida la restricción de Blackboard. Si Blackboard oculta el elemento a un estudiante, el lanzamiento LTI nunca ocurre para ese estudiante y no aparece en la vista de moderación.

#### Comportamiento del libro de calificaciones

FastComments no reporta calificaciones de vuelta a través de LTI Advantage Assignment and Grade Services. No se crea automáticamente ninguna columna de calificaciones para los elementos de contenido de FastComments.

Si su tenant de Blackboard está configurado para crear automáticamente una columna en el libro de calificaciones para cada nuevo elemento de contenido, independientemente de los metadatos de calificación, aparecerá una columna vacía de todos modos. Para ocultarla:

- Ultra: abra el **Gradebook**, haga clic en el encabezado de la columna, elija **Edit** y desactive **Show to students** junto con **Include in calculations**. O use **Delete** si su institución permite eliminar columnas para elementos sin calificar.
- Original: abra el **Grade Center**, haga clic en la flecha de la columna, elija **Hide from Users (on/off)** y, opcionalmente, **Hide from Instructor View** bajo **Column Organization**.

#### Lo que ven los estudiantes

Cuando un estudiante abre el elemento de FastComments o se desplaza hasta un bloque incrustado:

1. Blackboard lanza el mensaje LTI 1.3 a FastComments. El estudiante inicia sesión mediante SSO usando su identidad de Blackboard (nombre, correo electrónico, avatar, rol) sin ver un formulario de inicio de sesión.
2. El hilo de comentarios se renderiza en el iframe. Los hilos, las respuestas, las menciones y las reacciones están disponibles según la configuración del widget de comentarios configurada en FastComments.
3. Sus comentarios se atribuyen a su cuenta de Blackboard. Si el estudiante edita su nombre o foto en Blackboard más tarde, el siguiente lanzamiento actualiza el perfil de FastComments.

Mapeo de roles de Blackboard a FastComments:

- **System Administrator** y **Course Builder** se mapean a **admin** de FastComments.
- **Instructor** y **Teaching Assistant** se mapean a **moderator** de FastComments.
- **Student**, **Guest** y **Observer** se mapean a **commenter** de FastComments.

Los moderadores ven los controles de moderación (fijar, ocultar, banear, eliminar) en línea en cada comentario del hilo.

#### Restringir el acceso público (recomendado)

Por defecto, los datos de comentarios de FastComments son de lectura pública. Cualquiera que pueda adivinar la URL de un hilo o un endpoint de la API puede ver sus comentarios, incluso fuera de Blackboard. Para las discusiones de curso, casi con seguridad querrá restringir la visualización solo a los estudiantes inscritos.

Abra su <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalización del widget</a> y cree una regla con **Require SSO To View Comments** habilitado, luego establezca el nivel de seguridad en **Secure SSO** para que los hilos solo puedan cargarse a través del lanzamiento LTI firmado.

Consulte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para ver la guía completa, incluido cómo limitar la regla a un solo dominio o página.

#### Ámbito de los hilos

FastComments define el ámbito de cada hilo por **(host de Blackboard, ID del curso, ID del resource link)**. Dos elementos de FastComments en el mismo curso producen dos hilos. El mismo elemento copiado entre dos shells de curso (por ejemplo, mediante una copia de curso) produce dos hilos, porque Blackboard emite un nuevo ID de resource link durante la copia. Para mantener un hilo compartido entre copias de cursos, use Deep Linking con un URN de hilo explícito configurado en FastComments antes de lanzar la copia.

#### Problemas específicos de Blackboard

**Falta el mosaico de FastComments en el menú Build Content (Original) o en Content Market (Ultra).** El administrador aprobó la herramienta pero dejó una política de institución que bloquea la colocación correspondiente. Vaya a **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edite la entrada de FastComments y confirme que tanto **Course Content Tool** (Original) como **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) estén habilitados. Guarde y actualice la página del curso.

**Error "Tool not configured for this context" o "Tool is not deployed" al lanzar.** El ámbito de despliegue registrado durante el registro dinámico no coincide con el contexto de institución al que pertenece el curso. En la entrada del proveedor de herramientas de Blackboard, verifique que el **Deployment ID** coincida con lo que FastComments muestra en su página de configuración LTI 1.3 para este tenant. Si difieren, elimine la colocación y vuelva a ejecutar el registro dinámico desde una URL de registro nueva (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consígala aquí</a>).

**La altura del iframe parece fija o el contenido se corta.** Algunos tenants de Blackboard vienen con una Content Security Policy estricta que bloquea el postMessage predeterminado de redimensionamiento del iframe LTI. FastComments emite tanto el mensaje estilo Canvas `lti.frameResize` como el mensaje conforme a la especificación IMS `org.imsglobal.lti.frameResize` para maximizar la compatibilidad, pero una anulación de CSP a nivel de tenant bloquea el listener del padre. Pida a su administrador que confirme que `*.fastcomments.com` está en la lista de permitidos de herramientas LTI y que ningún encabezado CSP personalizado está eliminando los eventos postMessage. El redimensionamiento entonces funciona sin configuración adicional.

**La copia de curso duplica los hilos.** La copia de curso de Blackboard emite nuevos IDs de resource link para las colocaciones LTI, por lo que los cursos copiados comienzan con hilos vacíos. Esto es lo esperado. Si necesita que el curso copiado herede el hilo original, configure Deep Linking con un URN de hilo explícito antes de copiar, o contacte con el soporte de FastComments para remapear los IDs de hilos de forma masiva.

**El estudiante ve un error genérico de Blackboard al lanzar.** La causa es un claim `email` faltante o desactualizado. Confirme que la política de institución para FastComments tiene **Role**, **Name** y **Email Address** habilitados bajo **User Fields to Send**. Guarde y luego lance de nuevo en una sesión de navegador nueva.