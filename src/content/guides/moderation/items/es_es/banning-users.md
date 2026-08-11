Hay dos formas de prohibir a los usuarios comentar en su sitio con FastComments.

La primera es si ya conoce su correo electrónico, puede ingresarlo en la página <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">usuarios prohibidos</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lista de usuarios prohibidos bajo Comentarios Moderados, con las direcciones de correo electrónico prohibidas y un botón para agregar una nueva prohibición'; title='La página de usuarios prohibidos' app-screenshot-end]

Esta página se puede acceder a través de Comentarios Moderados -> Usuarios Prohibidos

Cuando vamos a prohibir a un usuario, podemos elegir un tipo, ya sea Permanente o Prohibición de Sombra Permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Nuevo formulario de prohibición con un campo de correo electrónico y una opción de tipo de prohibición: Permanente o Prohibición de Sombra Permanente'; title='Prohibiendo a un Usuario' app-screenshot-end]

La segunda forma de prohibir a un usuario es haciendo clic en el botón de prohibir que se coloca en cada comentario en la página de Moderación de Comentarios.

Al hacer clic en el botón de prohibir, se le presentarán algunas opciones, donde podemos especificar el tipo de prohibición y la duración.

### Alias de correo electrónico

Al prohibir a un usuario por correo electrónico, FastComments ignora automáticamente los alias `+`. Por ejemplo, prohibir `user+alias@gmail.com` también prohibirá `user@gmail.com` y cualquier otra variación con `+` de esa dirección, como `user+other@gmail.com`.

### Prohibiciones de sombra

Una prohibición de sombra es un tipo de prohibición que hace que parezca que el comentario o voto del usuario se guardó correctamente, cuando en realidad no fue así. Esto puede ser deseable en ciertas situaciones.

### Prohibición mediante dirección IP

A menos que un inquilino desee excluirse, FastComments admite la prohibición mediante IP almacenando una versión hash de la dirección IP del comentarista.

---