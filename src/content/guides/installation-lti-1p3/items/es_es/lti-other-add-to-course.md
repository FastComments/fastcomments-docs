Una vez que FastComments esté registrado en la plataforma, los instructores lo añaden al contenido del curso utilizando los flujos estándar de herramientas externas de la plataforma. Esta página cubre Sakai 23.x y Schoology Enterprise.

#### Restringir el acceso público (recomendado)

Por defecto, los datos de comentarios de FastComments son legibles públicamente en cualquiera de las dos plataformas. Cualquiera que pueda adivinar la URL del hilo o el endpoint de la API puede ver sus comentarios, incluso fuera de Sakai o Schoology. Para las discusiones de curso casi con seguridad querrá restringir la visualización solo a los estudiantes matriculados.

Abra su <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalización del widget</a> y cree una regla con **Require SSO To View Comments** habilitado, luego establezca el nivel de seguridad en **Secure SSO** para que los hilos solo puedan cargarse a través del lanzamiento LTI firmado.

Vea [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para el recorrido completo, incluyendo cómo limitar la regla a un único dominio o página.

#### Sakai

**1. Añadir FastComments a un sitio**

El mantenedor del sitio habilita la herramienta por sitio:

1. Abra el sitio y haga clic en **Site Info** en la navegación izquierda.
2. Haga clic en **Manage Tools**.
3. Desplácese a la lista **External Tools** y active **FastComments**.
4. Haga clic en **Continue**, revise la lista de herramientas y luego haga clic en **Finish**.

FastComments ahora aparece como un elemento en la navegación izquierda del sitio.

**2. Reordenar la entrada de la navegación izquierda**

Vaya a **Site Info** > **Tool Order**. Arrastre **FastComments** a la posición deseada y haga clic en **Save**. También puede renombrar la etiqueta de la navegación y ocultarla a los estudiantes desde esta pantalla.

**3. Insertar en línea en una página de Lessons**

Para colocar FastComments directamente dentro de una página de Lessons en lugar de como una herramienta independiente en la navegación izquierda:

1. Abra la herramienta **Lessons** en el sitio.
2. Haga clic en **Add Content** > **Add External Tool**.
3. Seleccione **FastComments** de la lista.
4. Si FastComments anunció Deep Linking durante el registro, Sakai abre el selector de contenido de la herramienta para que pueda elegir o etiquetar el hilo. Si Deep Linking no se anunció, Sakai inserta un enlace de lanzamiento por defecto.
5. Guarde el elemento de Lessons.

Cada instancia embebida obtiene su propio hilo, con alcance a ese enlace de recurso.

**4. Ajustes de permisos para el acceso de los estudiantes**

Sakai controla los lanzamientos de herramientas externas mediante Realms. Para confirmar que los estudiantes pueden lanzar FastComments:

1. Inicie sesión como administrador de Sakai y abra **Administration Workspace** > **Realms**.
2. Abra el realm correspondiente (por ejemplo, `!site.template.course` o el realm específico del sitio).
3. Confirme que el rol `access` tiene habilitado `lti.launch` y que los permisos de rol en el grupo **external.tools** están concedidos.
4. Guarde el realm.

Para anulaciones a nivel de sitio, el mantenedor puede ajustar la visibilidad de la herramienta por rol desde **Site Info** > **Tool Order** ocultando o mostrando FastComments por rol.

**5. Lo que ven los estudiantes**

Los estudiantes hacen clic en el elemento de la navegación izquierda de FastComments (o se desplazan hasta el bloque embebido de Lessons) y llegan directamente a la vista de comentarios en hilos. SSO es automático: Sakai envía la identidad del usuario en el lanzamiento LTI y FastComments los autentica con su cuenta de Sakai.

Mapeo de roles:

- Sakai `Instructor` -> moderador de FastComments
- Sakai `Admin` (admin en Administration Workspace) -> administrador de FastComments
- Sakai `Student` / `access` -> comentarista de FastComments

**6. Problemas conocidos de Sakai**

- **La herramienta no es visible en Manage Tools.** Si FastComments no aparece en la lista External Tools, el administrador de Sakai debe abrir el registro de herramientas (**Administration Workspace** > **External Tools** > **FastComments**) y establecer **Stealthed** en `false`. Las herramientas Stealthed están ocultas del selector Manage Tools por sitio.
- **Lanzamientos que fallan en navegadores con sesión compartida.** El token CSRF del portal de Sakai está ligado a la sesión del navegador. Si un estudiante ha iniciado sesión en dos sitios Sakai en diferentes pestañas o tiene una sesión obsoleta, el lanzamiento devuelve un 403. Solución: cierre otras pestañas de Sakai, cierre sesión, vuelva a iniciar sesión y vuelva a lanzar. Los administradores también pueden aumentar `sakai.csrf.token.cache.ttl` si esto sucede en todo el clúster.
- **Inserción en iframes.** Confirme que `lti.frameheight` en `sakai.properties` sea lo suficientemente grande (600 o más) para que el hilo de comentarios no se recorte dentro de una página de Lessons.

#### Schoology

Schoology Enterprise tiene dos escenarios de instalación. Confirme cuál aplica antes de añadir la herramienta a un curso.

**1. Dos escenarios de instalación**

- **(a) Instalación a nivel empresarial.** El Administrador del Sistema de Schoology instaló FastComments a nivel organizacional y lo asignó a todos los cursos o a plantillas de curso específicas. Los instructores se saltan la instalación y van directamente a "Add Materials".
- **(b) Instalación por el instructor.** El instructor instala la herramienta en un único curso desde **Course Options** > **External Tools** > **Install LTI Apps**. La instalación por parte del instructor requiere que el Administrador del Sistema haya aprobado primero la aplicación FastComments a nivel organizacional.

**2. Añadir FastComments como material del curso**

Dentro del curso:

1. Abra el curso y vaya a **Materials**.
2. Haga clic en **Add Materials** > **Add File/Link/External Tool**.
3. Elija **External Tool**.
4. Seleccione **FastComments** de la lista de herramientas registradas.
5. Establezca un **Name** (esto es lo que ven los estudiantes en la lista de materiales) y una **Description** opcional.
6. Deje **Enable Grading** (grade passback) **OFF**. FastComments no informa calificaciones de vuelta a Schoology, por lo que habilitar el envío de calificaciones crea una columna vacía en el libro de calificaciones.
7. Haga clic en **Submit**.

El material ahora aparece en la lista de materiales del curso y abre el hilo de FastComments al hacer clic.

**3. Inserción en línea mediante el editor Rich Text**

Si el Administrador del Sistema habilitó la colocación Deep Linking para FastComments durante el registro, los instructores pueden incrustar el hilo de comentarios dentro de cualquier campo Rich Text (instrucciones de la tarea, cuerpos de página, propuestas de discusión):

1. Abra el editor Rich Text en la página objetivo.
2. Haga clic en el icono **External Tool** (pieza de rompecabezas) en la barra de herramientas.
3. Elija **FastComments**.
4. Configure la inserción en el diálogo de deep-linking y haga clic en **Insert**.
5. Guarde la página.

Si el botón External Tool no aparece en el editor Rich Text, Deep Linking está deshabilitado para esta herramienta en este tenant. Vea los inconvenientes abajo.

**4. Visibilidad y asignación por secciones**

Schoology delimita la disponibilidad de la herramienta por sección a través de Course Options:

1. Desde el curso, haga clic en **Course Options** > **External Tools**.
2. Para cada aplicación LTI instalada, usted controla si está disponible para todas las secciones del curso o para secciones específicas.
3. Para restringir FastComments a ciertas secciones, desmarque las secciones que no deberían ver la herramienta.
4. El acceso a nivel de sección también controla qué secciones ven la entrada **Add Materials** > **External Tool** para FastComments.

**5. Lo que ven los estudiantes**

Los estudiantes hacen clic en el material de FastComments (o se desplazan hasta la inserción en línea) y acceden a la discusión en hilos. SSO es automático mediante el lanzamiento LTI de Schoology bajo su cuenta de Schoology.

Mapeo de roles:

- Schoology `Administrator` -> administrador de FastComments
- Schoology `Instructor` -> moderador de FastComments
- Schoology `Student` -> comentarista de FastComments

**6. Problemas conocidos de Schoology**

- **Solo Enterprise.** Las cuentas personales y gratuitas de Schoology no pueden instalar herramientas LTI 1.3. Si su tenant está en el nivel gratuito, la opción **External Tools** está ausente de Course Options. Actualice a Schoology Enterprise para usar FastComments.
- **Deep Linking deshabilitado por defecto en el tenant.** Algunos tenants de Schoology restringen la colocación Deep Linking a nivel org. Cuando esto ocurre, los instructores ven solo el flujo **Add Materials** > **External Tool** y no el botón External Tool en el editor Rich Text. Para habilitar la inserción en línea, el Administrador del Sistema debe ir a **System Settings** > **Integration** > **LTI 1.3** > **FastComments** y habilitar la colocación **Content Item / Deep Linking**, luego guardar.
- **Anulación de asignación por sección.** Si FastComments está asignado a nivel empresarial pero el instructor no puede verlo en **Add Materials**, la sección del curso está excluida en la asignación a nivel org. Pida al Administrador del Sistema que añada la sección a la asignación de la aplicación FastComments.
- **Nombre del material vs. identidad del hilo.** Renombrar el material en Schoology no mueve el hilo de comentarios. Los hilos están indexados por el LTI resource link ID, por lo que un cambio de nombre mantiene el mismo hilo; eliminar y volver a crear el material genera un hilo nuevo y vacío.