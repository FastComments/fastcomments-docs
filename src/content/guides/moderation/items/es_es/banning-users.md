Hay dos formas de prohibir a los usuarios comentar en su sitio con FastComments.

La primera es si ya conoce su correo electrónico, puede ingresarlo en la página de <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">usuarios prohibidos</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lista de usuarios prohibidos bajo Comentarios Moderados, con las direcciones de correo electrónico prohibidas y un botón para agregar una nueva prohibición'; title='Página de Usuarios Prohibidos' app-screenshot-end]

Esta página se puede acceder a través de Comentarios Moderados -> Usuarios Prohibidos

Cuando vamos a prohibir a un usuario, podemos elegir un tipo, ya sea Permanente o Prohibición de Sombra Permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Nuevo formulario de prohibición con un campo de correo electrónico y una opción de tipo de prohibición de Permanente o Prohibición de Sombra Permanente'; title='Prohibiendo a un Usuario' app-screenshot-end]

La segunda forma de prohibir a un usuario es haciendo clic en el botón de prohibir que se coloca en cada comentario en la página de Moderación de Comentarios.

Al hacer clic en el botón de prohibir, se le presentarán algunas opciones, donde podemos especificar el tipo de prohibición y la duración.

### Alias de Correo Electrónico

Al prohibir a un usuario por correo electrónico, FastComments ignora automáticamente los alias `+`. Por ejemplo, prohibir `user+alias@gmail.com` también prohibirá `user@gmail.com` y cualquier otra variación `+` de esa dirección, como `user+other@gmail.com`.

### Prohibiciones de Sombra

Una prohibición de sombra es un tipo de prohibición que hace que parezca que el comentario o voto del usuario se guardó correctamente, cuando en realidad no fue así. Esto puede ser deseable en ciertas situaciones.

### Prohibir mediante Dirección IP

A menos que un inquilino decida excluirse, FastComments admite la prohibición mediante IP almacenando una versión hash de la dirección IP del comentarista.

### Buscar Usuarios Prohibidos

Una vez que su lista crezca más de una o dos páginas, puede reducirla con la fila de búsqueda encima de la tabla.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Fila de búsqueda en la página de Usuarios Prohibidos con un menú desplegable Buscar Por, un menú desplegable Coincidencia y un campo de Valor'; title='Buscar Usuarios Prohibidos' app-screenshot-end]

Hay tres controles:

- **Search By** elige en qué campo buscar: Any Field, Email, Name, Banned By, o Banned For Saying. Los últimos cuatro corresponden a las columnas del mismo nombre en la tabla.
- **Match** elige cómo comparar. **Contains** encuentra su valor en cualquier parte del campo, y **Equals** coincide con todo el campo.
- **Value** es el texto a buscar.

Cada campo se compara sin distinguir mayúsculas y minúsculas, por lo que buscar `SPAMMER@EXAMPLE.COM` encuentra una prohibición almacenada como `spammer@example.com`.

Algunas cosas que vale la pena saber:

- **Banned For Saying** busca el texto del comentario que provocó la prohibición del usuario. Así es como encuentra a todos los usuarios prohibidos por una frase en particular.
- **Banned By** busca el nombre del moderador que emitió la prohibición, lo cual es útil para revisar las decisiones de otro moderador.
- Las prohibiciones comodín se almacenan con su `*`, por lo que una búsqueda **Contains** de `bademail.com` encuentra una prohibición `*@bademail.com`.
- **Name** coincide con el nombre mostrado en la columna Name, por lo que encuentra a un usuario incluso si ha cambiado su nombre desde que fue prohibido, y también si creó la prohibición ingresando una dirección de correo electrónico y no se registró un nombre en ese momento. El nombre registrado en la prohibición también coincide, por lo que buscar tanto el nombre antiguo como el actual funciona.
- **Any Field** busca el correo electrónico, nombre, moderador que prohibió y el texto del comentario prohibido juntos.

Su búsqueda forma parte de la URL de la página, por lo que puede compartir una lista filtrada con otros moderadores de la misma manera que comparte otros enlaces de moderación. Navegar por las páginas de resultados mantiene la búsqueda aplicada, iniciar una nueva búsqueda lo lleva a la primera página, y **Clear** devuelve la lista completa.