Hay dos formas de impedir que los usuarios comenten en su sitio con FastComments.

La primera es: si ya conoce su correo electrónico, puede introducirlo en la <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">usuarios baneados</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Esta página se puede acceder desde Moderar comentarios -> Usuarios baneados

Cuando vayamos a banear a un usuario, podemos elegir un tipo: Permanente o Baneo encubierto permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

La segunda forma de banear a un usuario es haciendo clic en el botón de baneo que aparece en cada comentario en la página de Moderación de comentarios.

Al hacer clic en el botón de baneo, se mostrarán algunas opciones donde podemos especificar el tipo y la duración del baneo.

### Alias de correo electrónico

Al banear a un usuario por correo electrónico, FastComments ignora automáticamente los alias con `+`. Por ejemplo, banear `user+alias@gmail.com` también baneará `user@gmail.com` y cualquier otra variante con `+` de esa dirección, como `user+other@gmail.com`.

### Baneos encubiertos

Un baneo encubierto es un tipo de baneo que hace que parezca que el comentario o voto del usuario se guardó correctamente, cuando en realidad no fue así. Esto puede ser deseable en ciertas situaciones.

### Baneo por dirección IP

A menos que un inquilino quiera optar por no participar, FastComments admite el baneo por IP almacenando una versión hash de la dirección IP del comentarista.