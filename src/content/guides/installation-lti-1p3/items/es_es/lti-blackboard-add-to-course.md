Una vez que un administrador haya registrado FastComments como una herramienta LTI 1.3 Advantage y aprobado las políticas de la institución, los instructores la añaden a los cursos mediante los puntos de colocación estándar de Blackboard. Los pasos exactos difieren entre Ultra Course View y Original Course View, por lo que se cubren ambos a continuación.

#### Ultra Course View

Ultra Course View es la opción predeterminada en Blackboard Learn SaaS desde 2026.

1. Abra el curso y vaya a la página **Course Content**.
2. Pase el cursor o toque donde desea que aparezca el hilo de comentarios en el índice y haga clic en el botón morado **+** (Add content).
3. Elija **Content Market**. El panel Content Market enumera todas las herramientas LTI aprobadas y las colocaciones de Building Block para su institución.
4. Busque el mosaico **FastComments** y haga clic en él. Blackboard crea un elemento de contenido en la posición donde abrió el menú **+**.
5. El elemento aparece en el índice como una entrada "Visible to students" por defecto para los instructores que tienen **Hide from students** desactivado como su valor predeterminado personal. Si su predeterminado es **Hidden**, el elemento se crea oculto y puede activar el selector de visibilidad en la fila del elemento cuando esté listo.
6. Para cambiar el nombre del elemento, haga clic en el título en el índice y escriba una nueva etiqueta. El título que ven los estudiantes en el índice es independiente del identificador del hilo de FastComments, por lo que renombrarlo es seguro en cualquier momento.

Si no ve **Content Market** como opción, su institución ha ocultado la colocación. También puede acceder al mismo selector a través de **More tools** en el mismo menú **+** bajo el grupo **LTI Tools**.

#### Original Course View

Original Course View sigue siendo compatible en Learn SaaS y sigue siendo la experiencia principal para sitios Learn 9.1 autohospedados en la línea de lanzamiento CU de Q4 2024.

1. Abra el curso y entre en un **Content Area** (por ejemplo, el área predeterminada **Information** o **Content** en el menú del curso).
2. Active **Edit Mode** con el interruptor en la esquina superior derecha de la página.
3. Haga clic en **Build Content** en la barra de acciones.
4. En el submenú **Learning Tools**, haga clic en **FastComments**. El submenú Learning Tools se rellena con las colocaciones de herramientas LTI 1.3 después de que un administrador registre la herramienta. Si no lo ve, consulte la sección de problemas comunes más abajo.
5. En el formulario **Create FastComments**, configure:
   - **Name**: la etiqueta que ven los estudiantes en el área de contenido.
   - **Description**: texto opcional mostrado encima del hilo incrustado.
   - **Permit Users to View this Content**: conmutador de disponibilidad Sí/No.
   - **Track Number of Views**: active si desea las estadísticas por elemento del Blackboard. FastComments tiene su propia analítica de forma independiente.
   - **Date and Time Restrictions**: ventanas opcionales **Display After** / **Display Until**.
6. Envíe. La herramienta aparece como un elemento clicable en el área de contenido.

#### Embedding Inside an Item or Document

En ambas vistas del curso, los instructores incrustan FastComments en línea dentro del cuerpo de un Item, Document o cualquier campo de texto enriquecido mediante el botón LTI Advantage del Editor de Contenido.

Ultra Course View:

1. Cree o edite un **Document**.
2. Haga clic en **Add content** dentro del cuerpo del documento donde desea que aparezca el hilo.
3. En la barra de herramientas del editor, abra el menú **Insert content** y haga clic en **Content Market** (el punto de entrada LTI Advantage / Deep Linking).
4. Elija **FastComments**. FastComments devuelve una payload de deep-link y Blackboard inserta un bloque incrustado en el cuerpo del documento en la posición del cursor.
5. Guarde el documento. Los estudiantes ven el hilo renderizado en línea mientras se desplazan.

Original Course View:

1. Edite cualquier elemento con un cuerpo de texto enriquecido.
2. En la barra de herramientas del Content Editor, haga clic en el icono más Add Content y elija **Content Market** (etiquetado como **Add Content from External Tool** en CUs antiguas de Q4 2024).
3. Elija **FastComments**. El editor inserta un bloque marcador de posición que hace referencia al recurso deep-linked.
4. Envíe el elemento.

Cada incrustación deep-link produce su propio hilo de FastComments, por lo que un Item con dos bloques FastComments incrustados tiene dos flujos de comentarios independientes.

#### Visibility, Release Conditions, and Group Restrictions

Los elementos de contenido FastComments se comportan como cualquier otro elemento de contenido de Blackboard para las reglas de control de acceso que se aplican sobre ellos.

- Ultra: haga clic en el selector de visibilidad en la fila (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilidad condicional admite ventanas de fecha/hora, reglas de rendimiento contra elementos del gradebook y reglas de miembro contra grupos del curso.
- Original: abra el menú contextual del elemento y elija **Adaptive Release** o **Adaptive Release: Advanced** para restringir la herramienta por fecha, membresía, calificación o estado de revisión. Use **Set Group Availability** en el elemento para restringir a grupos específicos del curso.

FastComments respeta lo que decida el control de Blackboard. Si Blackboard oculta el elemento a un estudiante, el lanzamiento LTI nunca ocurre para ese estudiante y no aparecen en la vista de moderador.

#### Gradebook Behavior

FastComments no informa calificaciones a través de LTI Advantage Assignment and Grade Services. No se crea automáticamente una columna de calificaciones para los elementos de contenido FastComments.

Si su tenant de Blackboard está configurado para crear automáticamente una columna del gradebook para cada nuevo elemento de contenido independientemente de los metadatos de calificación, de todos modos aparece una columna vacía. Para ocultarla:

- Ultra: abra el **Gradebook**, haga clic en el encabezado de la columna, elija **Edit**, y desactive **Show to students** y **Include in calculations**. O use **Delete** si su institución permite la eliminación de columnas para elementos no calificados.
- Original: abra el **Grade Center**, haga clic en el chevron de la columna, elija **Hide from Users (on/off)** y opcionalmente **Hide from Instructor View** bajo **Column Organization**.

#### What Students See

Cuando un estudiante abre el elemento FastComments o se desplaza hasta un bloque incrustado:

1. Blackboard lanza el mensaje LTI 1.3 a FastComments. El estudiante inicia sesión vía SSO usando su identidad de Blackboard (nombre, correo electrónico, avatar, rol) sin ver un formulario de inicio de sesión.
2. El hilo de comentarios se renderiza en el iframe. El threading, las respuestas, las menciones y las reacciones están disponibles según la configuración del widget de comentarios configurada en FastComments.
3. Sus comentarios se atribuyen a su cuenta de Blackboard. Si el estudiante edita su nombre o foto en Blackboard más tarde, el siguiente lanzamiento actualiza el perfil de FastComments.

Role mapping from Blackboard to FastComments:

- **System Administrator** and **Course Builder** map to FastComments **admin**.
- **Instructor** and **Teaching Assistant** map to FastComments **moderator**.
- **Student**, **Guest**, and **Observer** map to FastComments **commenter**.

Los moderadores ven controles de moderación (pin, hide, ban, delete) en línea en cada comentario del hilo.

#### Lock Down Public Access (Recommended)

Por defecto, los datos de comentarios de FastComments son legibles públicamente. Cualquiera que pueda adivinar la URL de un hilo o el endpoint de la API puede ver sus comentarios, incluso fuera de Blackboard. Para discusiones de curso probablemente desee restringir la visualización solo a los estudiantes matriculados.

Abra su <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalización del widget</a> y cree una regla con **Require SSO To View Comments** habilitado, luego establezca el nivel de seguridad en **Secure SSO** para que los hilos solo puedan cargarse mediante el lanzamiento LTI firmado.

Consulte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para el recorrido completo, incluyendo cómo limitar la regla a un único dominio o página.

#### Thread Scoping

FastComments delimita cada hilo por **(Blackboard host, course ID, resource link ID)**. Dos elementos FastComments en el mismo curso generan dos hilos. El mismo elemento copiado en dos instancias de curso (por ejemplo, mediante la copia del curso) genera dos hilos, porque Blackboard emite un nuevo resource link ID durante la copia. Para mantener un hilo compartido entre copias de curso, use Deep Linking con un URN de hilo explícito configurado en FastComments antes de ejecutar la copia.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** El administrador aprobó la herramienta pero dejó una política de la institución que bloquea la colocación relevante. Vaya a **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edite la entrada de FastComments y confirme que las colocaciones **Course Content Tool** (Original) y **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) estén habilitadas. Guarde y actualice la página del curso.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** El ámbito de despliegue registrado durante el registro dinámico no coincide con el contexto institucional al que pertenece el curso. En la entrada del proveedor de herramientas de Blackboard, verifique que el **Deployment ID** coincida con lo que FastComments muestra en su página de LTI 1.3 Configuration para este tenant. Si difieren, elimine la colocación y vuelva a ejecutar el registro dinámico desde una URL de registro nueva (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguela aquí</a>).

**Iframe height looks fixed or content gets cut off.** Algunos tenants de Blackboard incluyen una estricta Content Security Policy que bloquea el postMessage de redimensión de iframe LTI por defecto. FastComments emite tanto el mensaje al estilo Canvas `lti.frameResize` como el mensaje en forma de especificación IMS `org.imsglobal.lti.frameResize` para maximizar la compatibilidad, pero una anulación de CSP a nivel de tenant puede bloquear el listener padre. Pida a su administrador que confirme que `*.fastcomments.com` esté en la lista blanca de herramientas LTI y que ningún encabezado CSP personalizado esté eliminando eventos postMessage. La redimensión funcionará entonces sin configuración adicional.

**Course copy duplicates threads.** La copia de cursos de Blackboard emite nuevos resource link IDs para las colocaciones LTI, por lo que los cursos copiados comienzan con hilos vacíos. Esto es esperado. Si necesita que el curso copiado herede el hilo original, configure Deep Linking con un URN de hilo explícito antes de copiar, o contacte con el soporte de FastComments para remapear los IDs de hilo en bloque.

**Student sees a generic Blackboard error on launch.** La causa es una claim `email` faltante o desactualizada. Confirme que la política de la institución para FastComments tiene habilitados **Role**, **Name**, y **Email Address** bajo **User Fields to Send**. Guarde y luego inicie de nuevo en una sesión de navegador nueva.