Aunque el Soporte de FastComments puede ayudar con las migraciones, la mayoría pueden realizarse y supervisarse fácilmente sin ninguna intervención
del personal de soporte.

Admitimos de forma nativa la importación de exportes desde los siguientes proveedores:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

Al navegar [aquí](https://fastcomments.com/auth/my-account/manage-data/import) podemos subir el archivo que contiene los datos a migrar.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitorización de importaciones

FastComments utiliza un sistema de procesamiento de trabajos para procesar importaciones y exportaciones. Una vez que el sistema haya recogido su trabajo, este
informará periódicamente sobre el estado del trabajo en la interfaz de importación o exportación.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Tenga en cuenta que el estado de las importaciones y exportaciones puede ser visto por todos los administradores de la cuenta.

Si su trabajo falla, no se reiniciará automáticamente. La importación tendrá que intentarse de nuevo. Si alguna importación o exportación falla,
nuestros administradores del sistema son notificados automáticamente. Si identificamos un problema, nos pondremos en contacto con usted para ver si podemos ayudar.

### Volver a ejecutar la importación

Durante algunas migraciones, es necesario ejecutar la importación varias veces. Por ejemplo, es común hacer una primera pasada
de migración para pruebas, y luego ejecutar la importación nuevamente con los datos más recientes antes de activar el cambio.

Reimportar el mismo contenido **no creará duplicados**.

### Seguridad y expiración de los datos

Los archivos de importación no son accesibles mediante solicitudes externas de ninguna manera, y los archivos de importación se eliminan de nuestro sistema tan pronto como
la importación se completa.

---