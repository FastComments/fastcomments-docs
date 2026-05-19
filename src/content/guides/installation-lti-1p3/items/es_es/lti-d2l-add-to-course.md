Esta página explica cómo añadir FastComments a un curso de Brightspace después de que un administrador haya registrado la herramienta y creado una implementación. Si la herramienta aún no está registrada, consulte primero la guía de registro de D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incrustado como un tema de unidad en Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments ejecutándose dentro de una unidad de Brightspace, mostrando comentarios anidados y un selector de menciones @-nombre" />
</div>

Brightspace ofrece dos experiencias de creación de contenido: **Classic Content** y la **New Content Experience** (también llamada **Lessons**). Ambas exponen FastComments, pero las rutas de los menús difieren. Cada sección a continuación cubre ambas cuando divergen.

#### Localizar la herramienta FastComments

La herramienta FastComments aparece en dos lugares dentro del editor de contenido del curso:

1. El selector de actividades, accesible desde el botón **Add Existing** del módulo/unidad (etiquetado **Add Existing Activities** en versiones antiguas de Brightspace). FastComments aparece directamente en el selector en las versiones actuales de Brightspace; en versiones antiguas está anidado bajo un submenú **External Learning Tools**. Cualquiera de las rutas añade FastComments como un tema independiente.
2. El cuadro de diálogo **Insert Stuff** dentro del editor HTML, bajo **LTI Advantage**. Esto incrusta FastComments en línea en un tema HTML mediante el flujo de deep linking de LTI.

Si FastComments no aparece en ninguno de los selectores, la implementación no está habilitada para la unidad organizativa que contiene el curso. Pida al administrador de Brightspace que abra **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > herramienta FastComments > **View Deployments**, abra la implementación y añada la unidad organizativa del curso (o una unidad organizativa padre) bajo **Org Units**.

#### Añadir FastComments como un tema en un módulo

Classic Content:

1. Abra el curso y haga clic en **Content** en la barra de navegación.
2. Seleccione el módulo que debe contener la discusión (o cree uno mediante **Add a module**).
3. Haga clic en **Add Existing** (Brightspace antiguo: **Add Existing Activities** > **External Learning Tools**).
4. En el selector, haga clic en **FastComments**. Brightspace crea un tema en el módulo y le devuelve a la vista de contenido.
5. Haga clic en el nuevo tema. Cambie su nombre por algo descriptivo como `FastComments Discussion` usando el editor de título en línea.

New Content Experience (Lessons):

1. Abra el curso y haga clic en **Content**.
2. Abra la unidad y la lección que deben contener la discusión.
3. Haga clic en **Add** > **Existing Activity** y seleccione **FastComments** (Brightspace antiguo: anidado bajo **External Learning Tools**).
4. La actividad se añade a la lección.
5. Haga clic en el título de la actividad para renombrarla.

La primera vez que cualquier usuario (profesor o estudiante) abra el tema, FastComments inicializa el hilo para ese resource link. El hilo está vinculado al ID del resource link, por lo que renombrar o mover el tema no cambia qué hilo se carga.

#### Incrustar FastComments en línea en un tema HTML

Use este flujo cuando quiera que los comentarios aparezcan debajo de una lectura, vídeo u otro contenido dentro de la misma página del tema en lugar de como un tema separado.

1. Abra o cree un tema HTML en el módulo/lección.
2. Haga clic en **Edit HTML** para abrir el editor HTML de Brightspace.
3. Coloque el cursor donde debe aparecer el hilo de comentarios.
4. Haga clic en el botón **Insert Stuff** (icono de pieza de rompecabezas en la barra de herramientas del editor).
5. En el cuadro Insert Stuff, desplácese a **LTI Advantage** y haga clic en **FastComments**.
6. FastComments abre un selector de deep linking. Confirme la ubicación (las opciones por defecto funcionan para discusiones de contenido); haga clic en **Insert** o **Continue**.
7. Brightspace vuelve al editor HTML con un bloque de marcador de posición que representa el lanzamiento LTI. Haga clic en **Save and Close** en el tema.

Cuando se cargue el tema, Brightspace reemplazará el marcador por un iframe que lanza automáticamente FastComments mediante LTI. Los estudiantes verán el hilo de discusión en línea.

Un único tema HTML puede contener múltiples incrustaciones deep-linked de FastComments. Cada incrustación obtiene su propio hilo porque cada enlace profundo produce un resource link ID distinto.

#### Tema del módulo vs Enlace rápido inline

Elija el enfoque de **tema del módulo** cuando:

- La discusión sea la actividad principal para ese paso del módulo.
- Quiera que el tema aparezca en la tabla de contenidos de Brightspace, en el seguimiento de finalización y en Class Progress.

Elija el enfoque de **incrustación inline** cuando:

- Los comentarios deban situarse debajo de otro contenido en la misma página.
- No quiera un elemento separado rastreable por finalización en la tabla de contenidos.

#### Visibilidad, borrador y condiciones de publicación

Un nuevo tema de FastComments es visible para los estudiantes por defecto. Para ocultarlo mientras lo configura:

1. En el editor de contenido, haga clic en el título del tema (Classic) o en el menú de tres puntos de la actividad (New Content Experience).
2. Establezca el estado en **Draft** (Classic) o desactive la **Visibility** (New Content Experience).

Los temas en Draft son invisibles para los estudiantes. Los instructores y asistentes de enseñanza todavía los ven con una insignia “Draft”.

Para restringir el tema a un grupo o sección específicos:

1. Abra el tema.
2. Haga clic en el menú del título del tema > **Edit Properties In-place** (Classic) o **Edit** > **Restrictions** (New Content Experience).
3. Bajo **Release Conditions**, haga clic en **Create**.
4. Elija **Group enrollment** o **Section enrollment**, seleccione el grupo/sección y guarde.

Las condiciones de publicación se apilan con el propio mapeo de roles de FastComments. Los estudiantes que no puedan ver el tema no recibirán un lanzamiento LTI.

#### Qué ven los estudiantes en el primer lanzamiento

Cuando un estudiante hace clic en el tema (o carga un tema HTML con una incrustación):

1. Brightspace realiza el lanzamiento LTI 1.3 en segundo plano.
2. FastComments recibe el nombre del estudiante, correo electrónico, URL del avatar y rol en el LMS, y lo autentica automáticamente. No aparece un aviso de inicio de sesión de FastComments.
3. El hilo de comentarios para ese resource link se renderiza dentro del iframe de Brightspace.

Mapeo de roles en el lanzamiento:

- Brightspace `Administrator` se convierte en un administrador (admin) de FastComments para el hilo (moderación completa, eliminar, banear y acceso a configuración).
- Brightspace `Instructor` se convierte en un moderador (moderator) de FastComments (fijar, ocultar, eliminar, banear).
- Todos los demás roles (`Learner`, `TeachingAssistant`, etc.) se convierten en comentaristas estándar.

Los comentarios se atribuyen a la cuenta de Brightspace del estudiante. Si el estudiante edita su nombre o avatar en Brightspace, el siguiente lanzamiento LTI sincronizará el cambio.

#### Restringir el acceso público (Recomendado)

Por defecto, los datos de comentarios de FastComments son públicamente legibles. Cualquiera que pueda adivinar la URL del hilo o el endpoint de la API puede ver sus comentarios, incluso fuera de Brightspace. Para las discusiones de curso casi con toda seguridad querrá restringir la vista solo a los alumnos matriculados.

Abra su <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalización del widget</a> y cree una regla con **Require SSO To View Comments** habilitado, luego establezca el nivel de seguridad en **Secure SSO** para que los hilos solo puedan cargarse mediante el lanzamiento LTI firmado.

Vea [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para el recorrido completo, incluyendo cómo aplicar la regla a un único dominio o página.

#### Altura del iframe y redimensionado

FastComments emite el postMessage `org.imsglobal.lti.frameResize` en cada renderizado de hilo y en cambios de contenido (nuevo comentario, expandir respuestas). Brightspace escucha este mensaje y ajusta la altura del iframe para que el hilo no quede recortado y no se muestre una barra de desplazamiento interna.

Si el iframe se mantiene en una altura fija y corta:

- Confirme que el curso se carga mediante HTTPS. El listener de postMessage de Brightspace rechaza marcos de contenido mixto.
- Confirme que ninguna extensión del navegador esté bloqueando el canal postMessage.
- Para incrustaciones inline en un tema HTML, el HTML circundante no debe envolver el iframe en un contenedor de altura fija. Elimine cualquier `style="height: ..."` inline del elemento padre.

#### Peculiaridades específicas de Brightspace

**La herramienta no aparece en el selector Add Existing.** La implementación no está habilitada para la unidad organizativa de este curso. Un administrador debe añadir la unidad organizativa (o una unidad padre) a la lista **Org Units** de la implementación. El registro de la herramienta por sí solo no es suficiente; la implementación determina qué cursos ven la herramienta.

**Coincidencia de `deployment_id` en el lanzamiento.** FastComments TOFU-pins el primer `deployment_id` que ve para una registración. Si un administrador borra la implementación original y crea una nueva, los lanzamientos desde la nueva implementación son rechazados con un error de discrepancia de deployment. La solución es volver a registrar FastComments (genere una nueva URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguela aquí</a>) y ejecute el registro dinámico de nuevo); el registro de configuración antiguo se reemplazará.

**La herramienta se lanza pero muestra "Invalid LTI launch".** El curso está en una estructura de tenant/organización diferente de la que cubre la implementación, o la implementación se deshabilitó después del registro. Vuelva a comprobar **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > alternador **Enabled** y la lista de unidades organizativas de la implementación.

**Faltan nombres y roles dentro de FastComments.** Brightspace envía lanzamientos LTI con claims de Names and Role Provisioning Services (NRPS). Si un curso fue actualizado desde un enlace LTI 1.1 antiguo, el lanzamiento carece de los claims `name` y `email`. Vuelva a añadir el tema FastComments mediante **Add Existing** (no migre el enlace antiguo) para que el lanzamiento use LTI 1.3.

**La incrustación muestra una pantalla de inicio de sesión en lugar de auto-SSO.** El tema HTML se insertó como un `<iframe>` simple apuntando a FastComments en lugar de mediante **Insert Stuff** > **LTI Advantage**. Los iframes simples omiten el lanzamiento LTI y llevan a los usuarios a la página pública de FastComments. Elimine el iframe y vuelva a insertarlo mediante el flujo Insert Stuff.