Un subconjunto de acciones de moderación puede realizarse directamente desde el hilo de comentarios, sin tener que ir a la página de Moderación de Comentarios.

Cuando hayas iniciado sesión, haz clic en el botón de editar en la esquina superior derecha de un comentario. Deberías tener las siguientes opciones como moderador:

- **Fijar** ese comentario
- **Eliminar** ese comentario
- **Eliminar** ese comentario + **Banear al usuario** (Permanente o Sombra, más detalles más adelante)
- **Editar** ese comentario
- **Bloquear** o **Desbloquear** ese comentario (más detalles abajo)
- Marcar ese comentario como **Aprobado** (mostrarlo) o **No aprobado** (ocultarlo)
- Marcar ese comentario como **Spam** o **No Spam**

### Bloquear un comentario

Bloquear un comentario individual impide cualquier nueva respuesta a él, y también evita que el propio comentario sea editado o eliminado hasta que se desbloquee. Esto se aplica a todos, incluidos administradores y moderadores. Si necesitas editar o eliminar un comentario bloqueado, desbloquéalo primero, realiza el cambio y vuélvelo a bloquear si lo deseas.

Un icono de candado aparece en la esquina superior derecha de un comentario bloqueado para que los lectores puedan ver de un vistazo que el hilo está cerrado. Las entradas de menú Edit y Delete están ocultas para comentarios bloqueados tanto en el widget de comentarios como en la API pública (`PATCH` y `DELETE` devuelven `code: 'locked'` si se llaman contra un comentario bloqueado).

Dos excepciones intencionadas evitan el bloqueo, porque de otro modo dejarían datos huérfanos: cuando un usuario elimina toda su cuenta (sus comentarios se limpian independientemente del estado de bloqueo), y cuando un moderador banea a un usuario con la opción "eliminar todos los comentarios de este usuario" (la limpieza borra los bloqueos).

### Cerrar hilos de comentarios

Los moderadores y administradores pueden bloquear, o cerrar, hilos de comentarios, seleccionando `Close Thread` en el menú de tres puntos en la parte superior del área de comentarios, si han iniciado sesión. Pueden seleccionar `Re-Open Thread` más adelante, en cualquier momento, para reabrir la posibilidad de comentar.

Cerrar un hilo de comentarios impide nuevos comentarios, pero aún permite votar y que los usuarios eliminen sus comentarios si así lo desean.

Cerrar y reabrir hilos de comentarios afecta instantáneamente a todos los usuarios que estén viendo el hilo.

También puedes marcar un hilo como solo lectura, lo que elimina las opciones de votar y eliminar, creando una regla de personalización específicamente para esa página.

### Actualizaciones en vivo

Todas estas acciones actualizarán los hilos de comentarios de otros usuarios de inmediato sin que tengan que recargar la página. Sin embargo, las acciones de moderador como ocultar un comentario o marcarlo como spam, no eliminan el comentario de la pantalla **del moderador** para que, si es necesario, puedan deshacer la acción rápidamente. Para indicar que un comentario está oculto, se resaltará en comparación con los demás comentarios (el color del resaltado depende del motivo de la eliminación).

Por ejemplo, dados los usuarios `A (commenter)`, `B (Moderator 1)`, y `C (Moderator 2)`.

...y el siguiente escenario:

1. `User B (Moderator 1)` oculta un comentario.
2. Para `User A (commenter)` ese comentario se oculta de inmediato.
3. Para `User C (Moderator 2)` ese comentario se oculta de inmediato.
4. Para el usuario que realizó el cambio, `User B (Moderator 1)`, el comentario permanece en su pantalla, pero está resaltado como eliminado. Tienen la opción de deshacer su acción, en cuyo caso los otros usuarios verán la actualización, en vivo, nuevamente.