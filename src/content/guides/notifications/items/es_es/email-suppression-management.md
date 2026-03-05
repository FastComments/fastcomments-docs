Cuando los correos enviados por FastComments rebotan o son marcados como spam por el destinatario, el proveedor de correo añade esa dirección a una lista de supresión. FastComments sincroniza estas listas de supresión diariamente para que no se envíen más correos a direcciones que no pueden recibirlos.

Los usuarios y moderadores con direcciones de correo electrónico suprimidas no recibirán notificaciones por correo electrónico, incluidas notificaciones de respuesta, notificaciones de menciones, alertas de administrador y correos digest. Aparecerá una insignia roja "Correo suprimido" junto a los usuarios y moderadores afectados en la interfaz de administración.

#### Ver correos suprimidos

Los administradores del inquilino con permiso Administrar datos pueden ver los correos suprimidos en la página
[Suppressed Emails](https://fastcomments.com/auth/my-account/suppressed-emails), dentro de Administrar datos.

La página muestra una tabla con todas las direcciones de correo suprimidas asociadas con los usuarios, moderadores y comentaristas de su inquilino. Puede filtrar por dirección de correo utilizando el campo de búsqueda.

#### Eliminar una supresión

Para eliminar una supresión, haga clic en el botón **Eliminar** junto a la entrada en la tabla. Será llevado a una página de confirmación que muestra los detalles de la supresión. Haga clic en **Confirmar eliminación** para continuar.

Cuando se elimina una supresión, FastComments contacta con el proveedor de correo para desbloquear la dirección y elimina la marca de supresión de todos los registros de usuario y moderador asociados.

#### Límites de tasa

Para evitar abusos, las eliminaciones están sujetas a límites de tasa:

- Cada dirección de correo solo puede dejar de estar suprimida una vez cada 30 días.
- Cada inquilino puede realizar hasta 5 eliminaciones por mes calendario.

Si una supresión vuelve a aparecer después de la eliminación, significa que la dirección de correo rebotó o fue reportada como spam de nuevo. En ese caso, debe resolverse el problema subyacente de entregabilidad antes de intentar otra eliminación.