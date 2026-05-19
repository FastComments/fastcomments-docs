Esta guía cubre cómo añadir FastComments a un curso de Moodle 4.x después de que un administrador del sitio haya registrado la herramienta y la haya configurado para mostrarse en el selector de actividades. Si FastComments aún no está registrado, consulte primero la guía de registro de Moodle.

#### Abrir el curso en modo edición

1. Inicie sesión en Moodle como Profesor con permisos de edición (o superior) para el curso.
2. Abra el curso.
3. Active el **Modo de edición** usando el interruptor en la esquina superior derecha del encabezado del curso.

Moodle 4.x reemplazó el desplegable heredado "Añadir una actividad o recurso" que usaba la 3.x por un cuadro de diálogo de selección de actividades a pantalla completa. Moodle 4.5 mantiene el mismo selector pero añade una fila de favoritos/estrellas en la parte superior, por lo que anclar FastComments una vez facilita acceder a él en secciones posteriores.

#### Añadir la actividad FastComments

1. Desplácese a la sección del curso (tema o semana) donde pertenece la discusión.
2. Haga clic en **Añadir una actividad o recurso** al final de esa sección.
3. En el cuadro de diálogo del selector, seleccione **FastComments**. Si no lo ve, vaya a la sección de problemas conocidos más abajo.

Se abre el formulario de configuración de la actividad. Los campos que importan:

- **Activity name** (obligatorio). Se muestra en la página del curso y en el libro de calificaciones. Ejemplo: `Week 3 Discussion`.
- **Activity description**. Texto introductorio opcional que se renderiza por encima del hilo de comentarios.
- **Show description on course page**. Marque esto si desea que la descripción sea visible sin entrar en la actividad.
- **Preconfigured tool**. Configurado a `FastComments` (seleccionado automáticamente cuando se lanza desde el selector). No lo cambie.
- **Launch container**. Poner en **New window**. Vea la sección de problemas conocidos para entender por qué "Same window" falla en algunas implementaciones de Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Déjelos en blanco. El Registro Dinámico manejó estos valores a nivel del sitio.

Desplácese al final y haga clic en **Save and return to course** (o **Save and display** para abrir la actividad de inmediato).

La actividad aparece como una fila en la sección con el icono de FastComments. Los estudiantes hacen clic en la fila para abrir el hilo de comentarios.

#### Insertar FastComments en línea con el editor

Para un hilo dentro de una Página, capítulo de Libro, Lección u otro recurso que use el editor Atto o TinyMCE:

1. Abra el recurso en modo edición.
2. Coloque el cursor donde debería aparecer el hilo.
3. En la barra de herramientas del editor, haga clic en el botón **LTI** / **External tool**. En Atto está etiquetado como "Insert LTI Advantage content". En TinyMCE (predeterminado en Moodle 4.3+) está bajo el menú **More** como **External tools**.
4. Elija **FastComments** de la lista de herramientas.
5. FastComments abre un selector de enlace profundo. Confirme el título del hilo y haga clic en **Embed**.
6. El editor inserta un bloque marcador LTI. Guarde el recurso.

Cada instancia insertada es un hilo distinto identificado por el ID del elemento de contenido de enlace profundo, así que una Página con tres inserciones de FastComments obtiene tres hilos independientes.

#### Restricciones de acceso y configuración de grupos

Los ajustes estándar de actividad de Moodle se aplican a las actividades FastComments:

- **Common module settings** > **Group mode**. Configurar esto como **Separate groups** o **Visible groups** no divide FastComments en hilos por grupo por sí solo. El modo de grupo de Moodle solo filtra el libro de calificaciones y la lista de miembros. Para ejecutar un hilo separado por grupo, añada una actividad FastComments por grupo y use **Restrict access** para delimitar cada una.
- **Restrict access** > **Add restriction**. Admite las condiciones estándar de Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, y conjuntos de restricciones anidados. Use **Group** para bloquear una actividad FastComments a un único grupo.
- **Activity completion**. Establezca en **Students must view this activity to complete it** si desea seguimiento de finalización. FastComments actualmente no informa un evento de finalización de vuelta a Moodle más allá del lanzamiento.

#### Asignación de roles

FastComments lee la reclamación LTI `roles` que Moodle envía en cada lanzamiento y la asigna de la siguiente manera:

- Moodle **Manager** o **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** o **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> solo lectura

Los administradores pueden eliminar cualquier comentario, prohibir usuarios y editar la configuración del hilo. Los moderadores pueden eliminar y aprobar comentarios dentro del hilo al que se lanzaron. Los roles personalizados de Moodle heredan la asignación del arquetipo del que fueron clonados.

#### Lo que ven los estudiantes

Los estudiantes hacen clic en la actividad FastComments (o se desplazan hasta el bloque insertado dentro de una Página o Libro). Moodle envía su identidad a FastComments mediante el lanzamiento LTI:

- Sin pantalla de inicio de sesión. FastComments los autentica usando la cuenta de Moodle.
- Su nombre para mostrar, correo electrónico y avatar provienen de Moodle.
- El hilo está acotado a (sitio Moodle, curso, ID de enlace de recurso), por lo que la misma actividad duplicada en otro curso obtiene un hilo nuevo.
- Las respuestas en hilo, las votaciones y las notificaciones funcionan igual que en un hilo de FastComments independiente.

#### Restringir el acceso público (recomendado)

Por defecto, los datos de comentarios de FastComments son legibles públicamente. Cualquiera que pueda adivinar la URL del hilo o el endpoint de la API puede ver sus comentarios, incluso fuera de Moodle. Para discusiones de curso casi seguro que querrá restringir la visualización solo a estudiantes matriculados.

Abra su <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalización del widget</a> y cree una regla con **Require SSO To View Comments** habilitado, luego establezca el nivel de seguridad en **Secure SSO** para que los hilos solo puedan cargarse a través del lanzamiento LTI firmado.

Vea [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para la guía completa, incluyendo cómo delimitar la regla a un solo dominio o página.

#### Problemas conocidos de Moodle

**FastComments no aparece en el selector de actividades.** El administrador del sitio registró la herramienta pero no configuró **Tool configuration usage** a **Show in activity chooser and as a preconfigured tool**. Corrija esto en **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > icono de engranaje en el mosaico de FastComments.

**El lanzamiento falla o muestra un marco en blanco cuando está configurado en "Same window".** Las cookies de sesión de Moodle usan `SameSite=Lax` por defecto, y algunos navegadores las eliminan en el POST entre sitios que LTI 1.3 usa para regresar desde FastComments. Configure **Launch container** a **New window** en la actividad. Este es un requisito estricto para FastComments insertado dentro de una Página o Libro, ya que la ruta de lanzamiento incrustada en el editor siempre abre una ventana nueva.

**El reclamo `iss` es la URL del sitio Moodle, no un ID de tenant.** FastComments usa la URL del sitio Moodle (el valor de configuración `wwwroot`) como el issuer de LTI. Si su instancia de Moodle se traslada a un nuevo dominio o cambia `wwwroot`, los hilos existentes de FastComments permanecen vinculados al issuer antiguo y no coincidirán con los nuevos lanzamientos. Vuelva a registrar la herramienta contra la nueva URL y migre los hilos a través del administrador de FastComments si es necesario.

**Copia de seguridad y restauración de actividades.** Hacer una copia de seguridad de un curso y restaurarlo en un curso nuevo crea nuevos IDs de enlace de recurso, por lo que las actividades FastComments restauradas comienzan con hilos vacíos. El curso original conserva los hilos originales. Esto es un comportamiento intencionado, no un error.

**TinyMCE por defecto en Moodle 4.5.** Moodle 4.5 se entrega con TinyMCE como editor predeterminado para instalaciones nuevas. La ubicación del botón External tool está bajo el menú **More** (`...`) en lugar de la barra principal. Los sitios antiguos que actualizaron desde 4.1 conservan Atto a menos que un administrador cambie el predeterminado.