Esta página trata sobre cómo añadir FastComments a un curso de Brightspace después de que un administrador haya registrado la herramienta y creado una implementación. Si la herramienta aún no está registrada, consulte primero la guía de registro de D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incrustado como un tema de unidad en Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments ejecutándose dentro de una unidad de Brightspace, mostrando comentarios anidados y un selector de menciones @" />
</div>

Brightspace ofrece dos experiencias de creación de contenido: **Contenido clásico** y la **Nueva experiencia de contenido** (también llamada **Lecciones**). Ambas exponen FastComments, pero las rutas del menú difieren. Cada sección a continuación cubre ambas cuando divergen.

#### Localizar la herramienta FastComments

La herramienta FastComments aparece en dos lugares dentro del editor de contenido de un curso:

1. El selector de actividades, al que se accede desde el botón **Agregar existente** de un módulo/unidad (etiquetado **Agregar actividades existentes** en versiones antiguas de Brightspace). FastComments aparece directamente en el selector en las versiones actuales de Brightspace; las versiones antiguas lo anidan bajo un submenú **Herramientas de aprendizaje externas**. Cualquiera de las dos rutas añade FastComments como un tema independiente.
2. El cuadro de diálogo **Insertar contenido** dentro del editor HTML, bajo **LTI Advantage**. Esto incrusta FastComments en línea en un tema HTML mediante el flujo de enlace profundo LTI.

Si FastComments no aparece en ninguno de los selectores, la implementación no está habilitada para la unidad organizativa que contiene el curso. Pida a su administrador de Brightspace que abra **Herramientas de administración** > **Administrar extensibilidad** > **LTI Advantage** > herramienta FastComments > **Ver implementaciones**, abra la implementación y añada la unidad organizativa del curso (o una unidad organizativa superior) bajo **Unidades organizativas**.

#### Añadir FastComments como un tema en un módulo

Contenido clásico:

1. Abra el curso y haga clic en **Contenido** en la barra de navegación.
2. Seleccione el módulo que debe contener la discusión (o cree uno mediante **Agregar un módulo**).
3. Haga clic en **Agregar existente** (Brightspace antiguo: **Agregar actividades existentes** > **Herramientas de aprendizaje externas**).
4. En el selector, haga clic en **FastComments**. Brightspace crea un tema en el módulo y le devuelve a la vista de contenido.
5. Haga clic en el nuevo tema. Cambie el nombre a algo descriptivo como `FastComments Discussion` usando el editor de título en línea.

Nueva experiencia de contenido (Lecciones):

1. Abra el curso y haga clic en **Contenido**.
2. Abra la unidad y la lección que debe contener la discusión.
3. Haga clic en **Agregar** > **Actividad existente** y seleccione **FastComments** (Brightspace antiguo: anidado bajo **Herramientas de aprendizaje externas**).
4. La actividad se añade a la lección.
5. Haga clic en el título de la actividad para renombrarla.

La primera vez que cualquier usuario (instructor o estudiante) abra el tema, FastComments inicializa el hilo para ese enlace de recurso. El hilo está vinculado al ID de enlace de recurso, por lo que renombrar o mover el tema no cambia qué hilo se carga.

#### Incrustar FastComments en línea en un tema HTML

Use este flujo cuando quiera que los comentarios aparezcan debajo de una lectura, video u otro contenido dentro de la misma página del tema en lugar de como un tema separado.

1. Abra o cree un tema HTML en el módulo/lección.
2. Haga clic en **Editar HTML** para abrir el editor HTML de Brightspace.
3. Coloque el cursor donde debe aparecer el hilo de comentarios.
4. Haga clic en el botón **Insertar contenido** (icono de pieza de rompecabezas en la barra de herramientas del editor).
5. En el cuadro de diálogo Insertar contenido, desplácese hasta **LTI Advantage** y haga clic en **FastComments**.
6. FastComments abre un selector de enlace profundo. Confirme la ubicación (las opciones predeterminadas funcionan para discusiones de contenido); haga clic en **Insertar** o **Continuar**.
7. Brightspace regresa al editor HTML con un bloque marcador que representa el lanzamiento LTI. Haga clic en **Guardar y cerrar** en el tema.

Cuando se carga el tema, Brightspace reemplaza el marcador por un iframe que lanza automáticamente FastComments vía LTI. Los estudiantes ven el hilo de discusión en línea.

Un único tema HTML puede contener múltiples incrustaciones de FastComments mediante enlaces profundos. Cada incrustación obtiene su propio hilo porque cada enlace profundo produce un ID de enlace de recurso distinto.

#### Tema del módulo vs enlace rápido incrustado

Elija el enfoque de **tema del módulo** cuando:

- La discusión sea la actividad principal para ese paso en el módulo.
- Desee que el tema aparezca en la tabla de contenidos de Brightspace, en el seguimiento de finalización y en Class Progress.

Elija el enfoque de **incrustación en línea** cuando:

- Los comentarios deban situarse debajo de otro contenido en la misma página.
- No quiera un elemento separable con seguimiento de finalización en la tabla de contenidos.

#### Visibilidad, borrador y condiciones de publicación

Un nuevo tema de FastComments es visible para los estudiantes por defecto. Para ocultarlo mientras lo configura:

1. En el editor de contenido, haga clic en el título del tema (Contenido clásico) o en el menú de tres puntos de la actividad (Nueva experiencia de contenido).
2. Establezca el estado en **Borrador** (Contenido clásico) o desactive la **Visibilidad** (Nueva experiencia de contenido).

Los temas en borrador son invisibles para los estudiantes. Los instructores y asistentes de enseñanza todavía los ven con una insignia de "Borrador".

Para restringir el tema a un grupo o sección específicos:

1. Abra el tema.
2. Haga clic en el menú del título del tema > **Editar propiedades en el lugar** (Contenido clásico) o **Editar** > **Restricciones** (Nueva experiencia de contenido).
3. Bajo **Condiciones de publicación**, haga clic en **Crear**.
4. Elija **Matrícula de grupo** o **Matrícula de sección**, seleccione el grupo/sección y guarde.

Las condiciones de publicación se apilan con el propio mapeo de roles de FastComments. Los estudiantes que no pueden ver el tema no reciben un lanzamiento LTI.

#### Qué ven los estudiantes en el primer lanzamiento

Cuando un estudiante hace clic en el tema (o carga un tema HTML con una incrustación):

1. Brightspace realiza el lanzamiento LTI 1.3 en segundo plano.
2. FastComments recibe el nombre del estudiante, correo electrónico, URL del avatar y rol en el LMS, y lo conecta automáticamente. No aparece ningún aviso de inicio de sesión de FastComments.
3. El hilo de comentarios para ese enlace de recurso se renderiza dentro del iframe de Brightspace.

Mapeo de roles en el lanzamiento:

- Brightspace `Administrator` se convierte en un **administrador** de FastComments para el hilo (moderación completa, eliminar, prohibir y acceso a la configuración).
- Brightspace `Instructor` se convierte en un **moderador** de FastComments (fijar, ocultar, eliminar, prohibir).
- Todos los demás roles (`Learner`, `TeachingAssistant`, etc.) se convierten en comentaristas estándar.

Los comentarios se atribuyen a la cuenta de Brightspace del estudiante. Si el estudiante edita su nombre o avatar en Brightspace, el siguiente lanzamiento LTI sincroniza el cambio.

#### Altura del iframe y cambio de tamaño

FastComments emite el postMessage `org.imsglobal.lti.frameResize` en cada renderizado de hilo y en los cambios de contenido (nuevo comentario, expansión de respuestas). Brightspace escucha este mensaje y ajusta la altura del iframe para que el hilo no se corte y no muestre una barra de desplazamiento interna.

Si el iframe se queda con una altura fija y corta:

- Confirme que el curso se carga mediante HTTPS. El listener de postMessage de Brightspace rechaza frames de contenido mixto.
- Confirme que ninguna extensión del navegador está bloqueando el canal postMessage.
- Para incrustaciones en línea en un tema HTML, el HTML circundante no debe envolver el iframe en un contenedor de altura fija. Elimine cualquier `style="height: ..."` en línea del elemento padre.

#### Problemas específicos de Brightspace

**La herramienta no aparece en el selector Agregar existente.** La implementación no está habilitada para la unidad organizativa de este curso. Un administrador necesita añadir la unidad organizativa (o una superior) a la lista de **Unidades organizativas** de la implementación. El registro de la herramienta por sí solo no es suficiente; la implementación determina qué cursos ven la herramienta.

**`deployment_id` mismatch on launch.** FastComments fija por TOFU el primer `deployment_id` que ve para un registro. Si un administrador elimina la implementación original y crea una nueva, los lanzamientos desde la nueva implementación son rechazados con un error de coincidencia de implementación. La solución es volver a registrar FastComments (genere una nueva URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguela aquí</a>) y ejecute el Registro Dinámico de nuevo); el registro de configuración antiguo se reemplaza.

**La herramienta se lanza pero muestra "Invalid LTI launch".** El curso está en una estructura de inquilino/organización diferente a la que cubre la implementación, o la implementación se deshabilitó después del registro. Verifique nuevamente **Herramientas de administración** > **Administrar extensibilidad** > **LTI Advantage** > FastComments > conmutador **Habilitado** y la lista de unidades organizativas de la implementación.

**Faltan nombres y roles dentro de FastComments.** Brightspace envía lanzamientos LTI con claims de Names and Role Provisioning Services (NRPS). Si un curso se actualizó desde un enlace LTI 1.1 antiguo, el lanzamiento carece de claims `name` y `email`. Vuelva a añadir el tema de FastComments mediante **Agregar existente** (no migre el enlace antiguo) para que el lanzamiento utilice LTI 1.3.

**La incrustación muestra una pantalla de inicio de sesión en lugar de SSO automático.** El tema HTML se insertó como un `<iframe>` simple apuntando a FastComments en lugar de mediante **Insertar contenido** > **LTI Advantage**. Los iframes simples omiten el lanzamiento LTI y llevan a los usuarios a la página pública de FastComments. Elimine el iframe y vuelva a insertarlo mediante el flujo Insertar contenido.