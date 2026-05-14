Esta página trata sobre cómo agregar FastComments a un curso de Brightspace después de que un administrador haya registrado la herramienta y creado una implementación. Si la herramienta aún no está registrada, consulte primero la guía de registro de D2L.

Brightspace ofrece dos experiencias para crear contenido: **Classic Content** y la **New Content Experience** (también llamada **Lessons**). Ambas exponen FastComments, pero las rutas del menú difieren. Cada sección a continuación cubre ambas cuando divergen.

#### Localizar la herramienta FastComments

La herramienta FastComments aparece en dos lugares dentro del editor de contenido de un curso:

1. El selector de actividades, al que se accede desde el botón **Agregar existente** de un módulo/unidad (etiquetado **Agregar actividades existentes** en versiones antiguas de Brightspace). FastComments aparece directamente en el selector en las versiones actuales de Brightspace; en versiones antiguas se anida bajo un submenú **Herramientas de aprendizaje externas**. Cualquiera de las rutas añade FastComments como un tema independiente.
2. El cuadro de diálogo **Insertar elementos** dentro del editor HTML, bajo **LTI Advantage**. Esto incrusta FastComments en línea en un tema HTML mediante el flujo de deep linking de LTI.

Si FastComments no aparece en ninguno de los selectores, la implementación no está habilitada para la unidad organizativa que contiene el curso. Pida a su administrador de Brightspace que abra **Herramientas del administrador** > **Administrar extensibilidad** > **LTI Advantage** > herramienta FastComments > **Ver implementaciones**, abra la implementación y agregue la unidad organizativa del curso (o una unidad organizativa padre) bajo **Unidades de organización**.

#### Agregar FastComments como un tema en un módulo

Classic Content:

1. Abra el curso y haga clic en **Content** en la barra de navegación.
2. Seleccione el módulo que debe contener la discusión (o cree uno mediante **Agregar un módulo**).
3. Haga clic en **Agregar existente** (Brightspace antiguo: **Agregar actividades existentes** > **Herramientas de aprendizaje externas**).
4. En el selector, haga clic en **FastComments**. Brightspace crea un tema en el módulo y lo devuelve a la vista de contenido.
5. Haga clic en el nuevo tema. Cambie su nombre por algo descriptivo como `FastComments Discussion` usando el editor de título en línea.

New Content Experience (Lessons):

1. Abra el curso y haga clic en **Content**.
2. Abra la unidad y la lección que deben contener la discusión.
3. Haga clic en **Agregar** > **Actividad existente** y seleccione **FastComments** (Brightspace antiguo: anidado bajo **Herramientas de aprendizaje externas**).
4. La actividad se añade a la lección.
5. Haga clic en el título de la actividad para renombrarla.

La primera vez que cualquier usuario (instructor o estudiante) abre el tema, FastComments inicializa el hilo para ese resource link. El hilo está vinculado al ID del resource link, por lo que renombrar o mover el tema no cambia qué hilo se carga.

#### Incrustar FastComments en línea en un tema HTML

Use este flujo cuando quiera que los comentarios aparezcan debajo de una lectura, un video u otro contenido dentro de la misma página del tema en lugar de como un tema separado.

1. Abra o cree un tema HTML en el módulo/lección.
2. Haga clic en **Editar HTML** para abrir el editor HTML de Brightspace.
3. Coloque el cursor donde deba aparecer el hilo de comentarios.
4. Haga clic en el botón **Insertar elementos** (ícono de pieza de rompecabezas en la barra de herramientas del editor).
5. En el cuadro de diálogo Insertar elementos, desplácese a **LTI Advantage** y haga clic en **FastComments**.
6. FastComments abre un selector de deep linking. Confirme la colocación (las opciones predeterminadas funcionan para discusiones de contenido); haga clic en **Insertar** o **Continuar**.
7. Brightspace vuelve al editor HTML con un bloque de marcador de posición que representa el lanzamiento LTI. Haga clic en **Guardar y cerrar** en el tema.

Cuando se carga el tema, Brightspace reemplaza el marcador por un iframe que inicia automáticamente FastComments vía LTI. Los estudiantes ven el hilo de discusión en línea.

Un solo tema HTML puede contener múltiples incrustaciones deep-linked de FastComments. Cada incrustación obtiene su propio hilo porque cada deep link produce un resource link ID distinto.

#### Tema del módulo vs. Enlace rápido en línea

Elija el enfoque de **tema del módulo** cuando:

- La discusión es la actividad principal para ese paso en el módulo.
- Desea que el tema aparezca en la tabla de contenidos de Brightspace, el seguimiento de finalización y Class Progress.

Elija el enfoque de **incrustación en línea** cuando:

- Los comentarios deben estar debajo de otro contenido en la misma página.
- No desea un elemento separado en la tabla de contenidos que se pueda rastrear para finalización.

#### Visibilidad, estado Borrador y Condiciones de publicación

Un nuevo tema de FastComments es visible para los estudiantes por defecto. Para ocultarlo mientras lo configura:

1. En el editor de contenido, haga clic en el título del tema (Classic) o en el menú de tres puntos de la actividad (New Content Experience).
2. Establezca el estado en **Borrador** (Classic) o desactive la **Visibilidad** (New Content Experience).

Los temas en estado Borrador son invisibles para los estudiantes. Los instructores y los TAs aún los ven con una etiqueta "Draft".

Para restringir el tema a un grupo o sección específicos:

1. Abra el tema.
2. Haga clic en el menú del título del tema > **Editar propiedades en el lugar** (Classic) o **Editar** > **Restricciones** (New Content Experience).
3. Bajo **Condiciones de publicación**, haga clic en **Crear**.
4. Elija **Inscripción por grupo** o **Inscripción por sección**, seleccione el grupo/sección y guarde.

Las condiciones de publicación se acumulan con el propio mapeo de roles de FastComments. Los estudiantes que no pueden ver el tema no reciben un lanzamiento LTI.

#### Qué ven los estudiantes en el primer lanzamiento

Cuando un estudiante hace clic en el tema (o carga un tema HTML con una incrustación):

1. Brightspace realiza el lanzamiento LTI 1.3 en segundo plano.
2. FastComments recibe el nombre del estudiante, correo electrónico, URL del avatar y el rol en el LMS, y los autentica automáticamente. No hay un aviso de inicio de sesión de FastComments.
3. El hilo de comentarios para ese resource link se renderiza dentro del iframe de Brightspace.

Mapeo de roles en el lanzamiento:

- Brightspace `Administrator` se convierte en un admin de FastComments para el hilo (acceso completo a moderación, eliminación, bloqueo y configuración).
- Brightspace `Instructor` se convierte en un moderator de FastComments (fijar, ocultar, eliminar, bloquear).
- Todos los demás roles (`Learner`, `TeachingAssistant`, etc.) se convierten en comentaristas estándar.

Los comentarios se atribuyen a la cuenta de Brightspace del estudiante. Si el estudiante edita su nombre o avatar en Brightspace, el siguiente lanzamiento LTI sincroniza el cambio.

#### Altura del iframe y redimensionado

FastComments emite el postMessage `org.imsglobal.lti.frameResize` en cada renderizado del hilo y en los cambios de contenido (nuevo comentario, expandir respuestas). Brightspace escucha este mensaje y ajusta la altura del iframe para que el hilo no quede recortado y no muestre una barra de desplazamiento interna.

Si el iframe se mantiene con una altura corta y fija:

- Confirme que el curso se carga mediante HTTPS. El listener postMessage de Brightspace rechaza frames de mixed-content.
- Confirme que ninguna extensión del navegador esté bloqueando el canal postMessage.
- Para incrustaciones en línea en un tema HTML, el HTML circundante no debe envolver el iframe en un contenedor de altura fija. Elimine cualquier atributo inline `style="height: ..."` del elemento padre.

#### Problemas específicos de Brightspace

**La herramienta no aparece en el selector Agregar existente.** La implementación no está habilitada para la unidad organizativa de este curso. Un administrador debe agregar la unidad organizativa (o una unidad padre) a la lista **Org Units** de la implementación. El registro de la herramienta por sí solo no es suficiente; la implementación determina qué cursos ven la herramienta.

**`deployment_id` no coincide en el lanzamiento.** FastComments TOFU-pins el primer `deployment_id` que ve para una registración. Si un administrador elimina la implementación original y crea una nueva, los lanzamientos desde la nueva implementación son rechazados con un error de incompatibilidad de implementación. La solución es volver a registrar FastComments (generar una nueva URL de registración y ejecutar Dynamic Registration de nuevo); se reemplaza el registro de configuración antiguo.

**La herramienta se lanza pero muestra "Invalid LTI launch".** El curso está en una estructura de inquilino/organización diferente de la que cubre la implementación, o la implementación se deshabilitó después del registro. Verifique de nuevo **Herramientas del administrador** > **Administrar extensibilidad** > **LTI Advantage** > FastComments > el interruptor **Enabled** y la lista de unidades organizativas de la implementación.

**Faltan nombres y roles dentro de FastComments.** Brightspace envía lanzamientos LTI con claims de Names and Role Provisioning Services (NRPS). Si un curso se actualizó desde un enlace LTI 1.1 más antiguo, el lanzamiento carece de claims `name` y `email`. Vuelva a agregar el tema FastComments mediante **Agregar existente** (no migre el enlace antiguo) para que el lanzamiento use LTI 1.3.

**La incrustación muestra una pantalla de inicio de sesión en lugar de auto-SSO.** El tema HTML se insertó como un `<iframe>` simple apuntando a FastComments en lugar de mediante **Insertar elementos** > **LTI Advantage**. Los iframes simples omiten el lanzamiento LTI y llevan a los usuarios a la página pública de FastComments. Elimine el iframe y vuelva a insertar mediante el flujo Insertar elementos.