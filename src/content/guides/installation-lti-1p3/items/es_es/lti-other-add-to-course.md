Una vez que FastComments está registrado en la plataforma, los instructores lo añaden al contenido del curso usando los flujos estándar de herramientas externas de la plataforma. Esta página cubre Sakai 23.x y Schoology Enterprise.

#### Sakai

**1. Añadir FastComments a un sitio**

El mantenedor del sitio habilita la herramienta por sitio:

1. Abra el sitio y haga clic en **Información del sitio** en la navegación izquierda.
2. Haga clic en **Gestionar herramientas**.
3. Desplácese hasta la lista **Herramientas externas** y active **FastComments**.
4. Haga clic en **Continuar**, revise la lista de herramientas y luego haga clic en **Finalizar**.

FastComments ahora aparece como un elemento del menú izquierdo en el sitio.

**2. Reordenar la entrada del menú izquierdo**

Vaya a **Información del sitio** > **Orden de herramientas**. Arrastre **FastComments** a la posición deseada y haga clic en **Guardar**. También puede renombrar la etiqueta del menú y ocultarla a los estudiantes desde esta pantalla.

**3. Insertar en línea en una página de Lessons**

Para colocar FastComments directamente dentro de una página de Lessons en lugar de como una herramienta independiente en el menú izquierdo:

1. Abra la herramienta **Lessons** en el sitio.
2. Haga clic en **Agregar contenido** > **Agregar herramienta externa**.
3. Seleccione **FastComments** de la lista.
4. Si FastComments anunció Deep Linking durante el registro, Sakai abre el selector de contenido de la herramienta para que pueda elegir o etiquetar el hilo. Si no se anunció Deep Linking, Sakai inserta un enlace de lanzamiento predeterminado.
5. Guarde el elemento de Lessons.

Cada instancia incrustada obtiene su propio hilo, con alcance limitado a ese enlace de recurso.

**4. Ajustes de permisos para el acceso de estudiantes**

Sakai controla los lanzamientos de herramientas externas mediante Realms. Para confirmar que los estudiantes pueden lanzar FastComments:

1. Inicie sesión como administrador de Sakai y abra **Espacio de administración** > **Realms**.
2. Abra el realm correspondiente (por ejemplo, `!site.template.course` o el realm específico del sitio).
3. Confirme que el rol `access` tiene habilitado `lti.launch` y que los permisos de rol en el grupo **external.tools** están concedidos.
4. Guarde el realm.

Para anulaciones a nivel de sitio, el mantenedor puede ajustar la visibilidad de la herramienta por rol desde **Información del sitio** > **Orden de herramientas** ocultando o mostrando FastComments por rol.

**5. Lo que ven los estudiantes**

Los estudiantes hacen clic en el elemento del menú izquierdo de FastComments (o desplazan hasta el bloque incrustado en Lessons) y acceden directamente a la vista de hilo de comentarios. El SSO es automático: Sakai envía la identidad del usuario en el lanzamiento LTI y FastComments los autentica con su cuenta de Sakai.

Mapeo de roles:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin en Espacio de administración) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Problemas conocidos de Sakai**

- **Herramienta no visible en Gestionar herramientas.** Si FastComments no aparece en la lista de Herramientas externas, el administrador de Sakai debe abrir el registro de herramientas (**Espacio de administración** > **Herramientas externas** > **FastComments**) y establecer **Stealthed** en `false`. Las herramientas en modo Stealthed están ocultas del selector Gestionar herramientas por sitio.
- **Lanzamientos que fallan en navegadores con sesión compartida.** El token CSRF del portal de Sakai está vinculado a la sesión del navegador. Si un estudiante ha iniciado sesión en dos sitios de Sakai en pestañas diferentes o tiene una sesión obsoleta, el lanzamiento devuelve un 403. Solución: cierre otras pestañas de Sakai, cierre sesión, vuelva a iniciar sesión y relance. Los administradores también pueden aumentar `sakai.csrf.token.cache.ttl` si esto ocurre en todo el clúster.
- **Incrustación en frames.** Confirme que `lti.frameheight` en `sakai.properties` sea lo suficientemente grande (600 o más) para que el hilo de comentarios no quede recortado dentro de una página de Lessons.

#### Schoology

Schoology Enterprise tiene dos escenarios de instalación. Confirme cuál aplica antes de añadir la herramienta a un curso.

**1. Dos escenarios de instalación**

- **(a) Instalación a nivel empresarial.** El administrador del sistema de Schoology instaló FastComments a nivel de la organización y lo asignó a todos los cursos o a plantillas de curso específicas. Los instructores omiten la instalación y van directamente a "Agregar materiales".
- **(b) Auto-instalación por el instructor.** El instructor instala la herramienta en un solo curso desde **Opciones del curso** > **Herramientas externas** > **Instalar aplicaciones LTI**. La auto-instalación requiere que el administrador del sistema haya aprobado primero la aplicación FastComments a nivel de la organización.

**2. Añadir FastComments como material del curso**

Dentro del curso:

1. Abra el curso y vaya a **Materiales**.
2. Haga clic en **Agregar materiales** > **Agregar archivo/enlace/herramienta externa**.
3. Elija **Herramienta externa**.
4. Seleccione **FastComments** de la lista de herramientas registradas.
5. Configure un **Nombre** (esto es lo que ven los estudiantes en la lista de materiales) y una **Descripción** opcional.
6. Deje **Habilitar calificaciones** (envío de calificaciones) **APAGADO**. FastComments no informa calificaciones a Schoology, por lo que habilitar el envío de calificaciones crea una columna vacía en el libro de calificaciones.
7. Haga clic en **Enviar**.

El material ahora aparece en la lista de materiales del curso y abre el hilo de FastComments al hacer clic.

**3. Incrustación en línea mediante el editor de texto enriquecido**

Si el administrador del sistema habilitó la colocación Deep Linking para FastComments durante el registro, los instructores pueden incrustar el hilo de comentarios dentro de cualquier campo de Texto enriquecido (instrucciones de la tarea, cuerpos de página, indicaciones de discusión):

1. Abra el editor de Texto enriquecido en la página de destino.
2. Haga clic en el icono **Herramienta externa** (pieza de rompecabezas) en la barra de herramientas.
3. Elija **FastComments**.
4. Configure la incrustación en el diálogo de deep-linking y haga clic en **Insertar**.
5. Guarde la página.

Si el botón Herramienta externa no aparece en el editor de Texto enriquecido, Deep Linking está deshabilitado para esta herramienta en ese tenant. Vea las advertencias a continuación.

**4. Visibilidad y asignaciones por secciones**

Schoology limita la disponibilidad de la herramienta por sección a través de Opciones del curso:

1. Desde el curso, haga clic en **Opciones del curso** > **Herramientas externas**.
2. Para cada aplicación LTI instalada, usted controla si está disponible para todas las secciones del curso o para secciones específicas.
3. Para restringir FastComments a ciertas secciones, desmarque las secciones que no deberían ver la herramienta.
4. El acceso a nivel de sección también determina qué secciones ven la entrada **Agregar materiales** > **Herramienta externa** para FastComments.

**5. Lo que ven los estudiantes**

Los estudiantes hacen clic en el material de FastComments (o desplazan hasta la incrustación en línea) y acceden a la discusión en hilo. El SSO es automático mediante el lanzamiento LTI de Schoology bajo su cuenta de Schoology.

Mapeo de roles:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Problemas conocidos de Schoology**

- **Solo Enterprise.** Las cuentas personales y gratuitas de Schoology no pueden instalar herramientas LTI 1.3. Si su tenant está en el nivel gratuito, la opción **Herramientas externas** no aparece en Opciones del curso. Actualice a Schoology Enterprise para usar FastComments.
- **Deep Linking deshabilitado por defecto en el tenant.** Algunos tenants de Schoology restringen la colocación Deep Linking a nivel organizacional. Cuando esto ocurre, los instructores ven solo el flujo **Agregar materiales** > **Herramienta externa** y no el botón Herramienta externa en el editor de Texto enriquecido. Para habilitar la incrustación en línea, el administrador del sistema debe ir a **Configuración del sistema** > **Integración** > **LTI 1.3** > **FastComments** y activar la colocación **Content Item / Deep Linking**, luego guardar.
- **Anulación de asignación por sección.** Si FastComments está asignado a nivel empresarial pero el instructor no puede verlo en **Agregar materiales**, la sección del curso está excluida en la asignación a nivel organizacional. Pida al administrador del sistema que añada la sección a la asignación de la aplicación FastComments.
- **Nombre del material vs. identidad del hilo.** Cambiar el nombre del material en Schoology no mueve el hilo de comentarios. Los hilos se identifican por el ID de enlace de recurso LTI, por lo que renombrar mantiene el mismo hilo; eliminar y volver a crear el material crea un hilo nuevo y vacío.