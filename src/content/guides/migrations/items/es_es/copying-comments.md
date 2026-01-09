En caso de que sea necesario mover datos, FastComments ofrece una herramienta de autoservicio para mover comentarios entre páginas y artículos.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Rellenando los campos "From"

Para decidir desde dónde mover los comentarios, simplemente necesitamos conocer el `URL ID` de origen.

Si no estás pasando un valor para `urlId` en la configuración del widget de comentarios, entonces esta será una versión "limpia" de la URL de la página.

Puedes ver qué valores tienen tus comentarios para `URL ID` exportándolos.

### Rellenando los campos "To"

Para decidir a dónde mover los comentarios, necesitamos conocer el `URL ID` de destino y la `URL`.

El `URL ID` será el contenedor en el que se colocará el comentario. El campo `URL` se usa para que puedas navegar directamente al comentario desde correos electrónicos y las herramientas de moderación.

#### WordPress

Si estás usando WordPress, por ejemplo introducirías los IDs de artículo en los campos To/From `URL ID` en la herramienta de migración, en lugar de una URL.