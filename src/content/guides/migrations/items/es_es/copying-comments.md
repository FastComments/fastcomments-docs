En caso de que sea necesario mover datos, FastComments ofrece una herramienta de autoservicio para mover comentarios entre páginas y artículos.

Así es como se ve el formulario de copia de comentarios:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Formulario de copia de comentarios con el campo From URL ID y los campos To URL ID y URL'; title='El formulario de copia de comentarios' app-screenshot-end]

### Rellenar los campos "From"

Para decidir de dónde mover los comentarios, simplemente necesitamos conocer el `URL ID` de origen.

Si no está pasando un valor para `urlId` en la configuración del widget de comentarios, entonces esto será una versión "limpia" de la URL de la página.

Puede ver qué valores tienen sus comentarios para `URL ID` exportándolos.

### Rellenar los campos "To"

Para decidir a dónde mover los comentarios, necesitamos conocer el `URL ID` y la `URL` de destino.

El `URL ID` será el contenedor en el que se coloca el comentario. El campo `URL` se utiliza para que pueda navegar directamente al comentario desde correos electrónicos y herramientas de moderación.

#### WordPress

Si está usando WordPress, por ejemplo ingresaría los IDs de los artículos en los campos `URL ID` To/From de la herramienta de migración, en lugar de una URL.