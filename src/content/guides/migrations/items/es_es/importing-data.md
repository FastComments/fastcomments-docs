---
Mientras el soporte de FastComments puede ayudar con las migraciones, la mayoría pueden realizarse y monitorearse fácilmente sin ninguna intervención del personal de soporte.

Soportamos nativamente la importación de exportaciones de los siguientes proveedores:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

Al navegar [aquí](https://fastcomments.com/auth/my-account/manage-data/import) podemos subir el archivo que contiene los datos a migrar.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='El formulario de la página de importación' app-screenshot-end]

### Monitoreo de importaciones

FastComments utiliza un sistema de procesamiento de trabajos para manejar importaciones y exportaciones. Una vez que el sistema haya tomado su trabajo, informará periódicamente el estado del mismo en la interfaz de importación o exportación.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Estado del trabajo de importación' app-screenshot-end]

Tenga en cuenta que el estado de las importaciones y exportaciones es visible para todos los administradores de la cuenta.

Si su trabajo falla, no se reiniciará automáticamente. La importación deberá intentarse nuevamente. Si alguna importación o exportación falla, los administradores de nuestro sistema son notificados automáticamente. Si identificamos un problema, nos pondremos en contacto con usted para ver si podemos ayudar.

### Reejecutar la importación

Durante algunas migraciones, es necesario ejecutar la importación varias veces. Por ejemplo, es común realizar una primera pasada de migración para pruebas y luego ejecutar la importación nuevamente con los datos más recientes antes de activar la funcionalidad.

Reimportar el mismo contenido **no creará duplicados**.

### Seguridad de datos y expiración

Los archivos de importación no son accesibles mediante solicitudes externas de ninguna manera, y los archivos de importación se eliminan de nuestro sistema tan pronto como la importación se completa.

---