#### Mención de Usuarios en Otros Grupos

Si dos usuarios pertenecen a dos conjuntos diferentes de grupos, y no hay intersección, no podrán `@mention` entre sí.

Si un usuario escribe manualmente un `@mention` y envía su comentario, permanecerá como texto sin formato. El otro usuario no será etiquetado.

#### Mantenimiento de los Grupos

`Groups` se definen mediante los recursos API `Pages` y `SSOUsers`, respectivamente.

La API `Pages` puede invocarse para definir el conjunto de grupos permitidos para acceder a la página. Por defecto, todos los grupos, y los usuarios que no pertenecen a un grupo, tienen acceso.

De forma similar, la API `SSOUsers` puede invocarse para definir los grupos asociados a cada usuario.

Para ambos recursos, no hay limitaciones respecto a cuándo se pueden establecer o actualizar los grupos.

Si solo se desea impedir que los usuarios se `@mention`en entre sí, entonces no es necesario tener en cuenta `Pages`.

##### ¡Nota!

Definir y actualizar los grupos de usuarios SSO no requiere usar la API, y en su lugar puede actualizarse automáticamente definiendo los ids de grupo en el payload SSO pasado al widget de comentarios. Sin embargo, para listas grandes de grupos, esto no se recomienda ya que el usuario tendría que enviar este payload en cada carga de página.