Esta guía cubre cómo añadir FastComments a un curso de Moodle 4.x después de que un administrador del sitio haya registrado la herramienta y la haya configurado para mostrarse en el selector de actividades. Si FastComments aún no está registrado, consulte primero la guía de registro de Moodle.

#### Abra el curso en modo de edición

1. Inicie sesión en Moodle como Profesor con permisos de edición (o superior) para el curso.
2. Abra el curso.
3. Active el **modo de edición** usando el interruptor en la esquina superior derecha del encabezado del curso.

Moodle 4.x reemplazó el antiguo desplegable "Add an activity or resource" que usaba la 3.x por un cuadro de diálogo de selección de actividades a pantalla completa. Moodle 4.5 mantiene el mismo selector pero añade una fila de favoritos con estrellas en la parte superior, por lo que fijar FastComments una vez facilita su acceso en secciones posteriores.

#### Añadir la actividad FastComments

1. Desplácese hasta la sección del curso (tema o semana) donde corresponde la discusión.
2. Haga clic en **Add an activity or resource** al final de esa sección.
3. En el cuadro de diálogo del selector, seleccione **FastComments**. Si no lo ve, vaya a la sección de incidencias más abajo.

Se abre el formulario de configuración de la actividad. Los campos que importan:

- **Activity name** (requerido). Se muestra en la página del curso y en el libro de calificaciones. Ejemplo: `Week 3 Discussion`.
- **Activity description**. Texto introductorio opcional que se muestra encima del hilo de comentarios.
- **Show description on course page**. Márquelo si desea que la descripción sea visible sin entrar en la actividad.
- **Preconfigured tool**. Ajustado a `FastComments` (seleccionado automáticamente cuando se lanza desde el selector). No lo cambie.
- **Launch container**. Ajuste a **New window**. Vea la sección de incidencias para saber por qué "Same window" falla en algunas implantaciones de Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Déjelos en blanco. El Registro Dinámico (Dynamic Registration) se encargó de estos a nivel del sitio.

Desplácese al final y haga clic en **Save and return to course** (o **Save and display** para abrir la actividad de inmediato).

La actividad aparece como una fila en la sección con el icono de FastComments. Los estudiantes hacen clic en la fila para abrir el hilo de comentarios.

#### Incrustar FastComments en línea con el editor

Para un hilo dentro de una Página, capítulo de Libro, Lección o cualquier otro recurso que use el editor Atto o TinyMCE:

1. Abra el recurso en modo de edición.
2. Coloque el cursor donde debe aparecer el hilo.
3. En la barra de herramientas del editor, haga clic en el botón **LTI** / **External tool**. En Atto aparece etiquetado como "Insert LTI Advantage content". En TinyMCE (por defecto en Moodle 4.3+) está bajo el menú **More** como **External tools**.
4. Elija **FastComments** de la lista de herramientas.
5. FastComments abre un selector de enlace profundo. Confirme el título del hilo y haga clic en **Embed**.
6. El editor inserta un bloque marcador LTI. Guarde el recurso.

Cada instancia incrustada es un hilo distinto identificado por el ID de elemento de contenido del enlace profundo, por lo que una Página con tres incrustaciones de FastComments tendrá tres hilos independientes.

#### Restricciones de acceso y configuración de grupos

Las opciones estándar de actividad de Moodle se aplican a las actividades FastComments:

- **Common module settings** > **Group mode**. Configurarlo en **Separate groups** o **Visible groups** no divide automáticamente FastComments en hilos por grupo. El modo de grupo de Moodle solo filtra el libro de calificaciones y la lista de miembros. Para ejecutar un hilo separado por grupo, añada una actividad FastComments por grupo y use **Restrict access** para limitar cada una.
- **Restrict access** > **Add restriction**. Admite las condiciones estándar de Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, y conjuntos anidados de restricciones. Use **Group** para bloquear una actividad FastComments a un solo grupo.
- **Activity completion**. Ajuste a **Students must view this activity to complete it** si desea el seguimiento de finalización. FastComments actualmente no informa un evento de finalización de vuelta a Moodle más allá del lanzamiento.

#### Asignación de roles

FastComments lee el reclamo LTI `roles` que Moodle envía en cada lanzamiento y lo asigna de la siguiente manera:

- Moodle **Manager** o **Administrador del sitio** -> FastComments **admin**
- Moodle **Editing teacher** o **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> solo lectura

Los administradores pueden eliminar cualquier comentario, prohibir usuarios y editar la configuración del hilo. Los moderadores pueden eliminar y aprobar comentarios dentro del hilo en el que se lanzaron. Los roles personalizados de Moodle heredan la asignación del arquetipo del que fueron clonados.

#### Lo que ven los estudiantes

Los estudiantes hacen clic en la actividad FastComments (o se desplazan hasta el bloque incrustado dentro de una Página o Libro). Moodle envía su identidad a FastComments mediante el lanzamiento LTI:

- Sin pantalla de inicio de sesión. FastComments les autentica usando la cuenta de Moodle.
- Su nombre visible, correo electrónico y avatar provienen de Moodle.
- El hilo está limitado a `(Moodle site, course, resource link ID)`, por lo que la misma actividad duplicada en otro curso obtiene un hilo nuevo.
- Las respuestas en hilo, las votaciones y las notificaciones funcionan igual que en un hilo independiente de FastComments.

#### Incidencias de Moodle

**FastComments no aparece en el selector de actividades.** El administrador del sitio registró la herramienta pero no configuró **Tool configuration usage** a **Show in activity chooser and as a preconfigured tool**. Corrija esto en **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > icono de engranaje en la ficha de FastComments.

**El lanzamiento falla o muestra un marco en blanco cuando está configurado en "Same window".** Las cookies de sesión de Moodle usan `SameSite=Lax` por defecto, y algunos navegadores las eliminan en el POST entre sitios que LTI 1.3 usa para volver desde FastComments. Configure **Launch container** en **New window** en la actividad. Esto es un requisito estricto para FastComments incrustado dentro de una Página o Libro, ya que la ruta de lanzamiento incrustada por el editor siempre abre una nueva ventana.

**El reclamo `iss` es la URL del sitio Moodle, no un ID de tenant.** FastComments usa la URL del sitio Moodle (el valor de configuración `wwwroot`) como el emisor LTI. Si su instancia de Moodle se mueve a un nuevo dominio o cambia `wwwroot`, los hilos existentes de FastComments permanecerán ligados al antiguo emisor y no coincidirán con los lanzamientos nuevos. Vuelva a registrar la herramienta con la nueva URL y migre los hilos mediante el administrador de FastComments si es necesario.

**Copia y restauración de actividades.** Hacer una copia de seguridad de un curso y restaurarla en un curso nuevo crea nuevos IDs de enlace de recurso, por lo que las actividades FastComments restauradas comienzan con hilos vacíos. El curso original conserva los hilos originales. Esto es un comportamiento intencionado, no un error.

**TinyMCE por defecto en Moodle 4.5.** Moodle 4.5 se distribuye con TinyMCE como editor predeterminado para instalaciones nuevas. La ubicación del botón External tool está bajo el menú **More** (`...`) en lugar de la barra principal. Los sitios más antiguos que actualizaron desde 4.1 conservan Atto a menos que un administrador cambie el predeterminado.