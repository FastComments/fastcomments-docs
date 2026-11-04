Si bien el Soporte de FastComments puede ayudar con las migraciones, la mayoría pueden realizarse y monitorearse fácilmente sin ninguna intervención
del personal de soporte.

Soportamos de forma nativa la importación de exportaciones de los siguientes proveedores:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (a través del plugin, o una exportación XML o CSV)
- AnyComment (A través de Importar/Exportar de WordPress)

Navegando [aquí](https://fastcomments.com/auth/my-account/manage-data/import) podemos subir el archivo que contiene los datos a migrar.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='Página de importación de FastComments con los campos de selección de proveedor y subida de archivo para un archivo de exportación'; title='El Formulario de la Página de Importación' app-screenshot-end]

### Monitoreo de Importaciones

FastComments utiliza un sistema de procesamiento de trabajos para procesar importaciones y exportaciones. Una vez que el sistema ha tomado su trabajo, este
reportará periódicamente el estado del trabajo en la interfaz de importación o exportación.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Página de importación mostrando un trabajo de importación en ejecución y el estado reportado por el sistema de procesamiento de trabajos'; title='Estado del Trabajo de Importación' app-screenshot-end]

Tenga en cuenta que el estado de las Importaciones y Exportaciones es visible para todos los administradores de la cuenta.

Si su trabajo falla, no se reiniciará automáticamente. La importación tendrá que intentarse de nuevo. Si alguna importación o exportación falla,
nuestros administradores del sistema son notificados automáticamente. Si identificamos un problema, nos pondremos en contacto con usted para ver si podemos ayudar.

### Re-ejecutando la Importación

Durante algunas migraciones, es necesario ejecutar la importación varias veces. Por ejemplo, es común hacer una primera pasada de
migración para pruebas, y luego ejecutar la importación de nuevo con los datos más recientes antes de cambiar el interruptor.

Reimportar el mismo contenido **no creará duplicados**.

### Seguridad y Expiración de los Datos

Los archivos de importación no son accesibles de ninguna manera a través de solicitudes externas, y los archivos de importación se eliminan de nuestro sistema tan pronto como
la importación se completa.