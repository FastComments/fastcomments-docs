---
FastComments funciona con sitios exclusivos para miembros utilizando lo que se llama SSO, o inicio de sesión único. Tus miembros inician sesión en tu sitio de WordPress, pero
no tienen que preocuparse por crear una cuenta en FastComments, ni iniciar sesión con redes sociales, para comentar. Si tus miembros (incluidos los administradores) han iniciado sesión en
tu sitio de WordPress, podrán comentar. Tus administradores y moderadores también podrán moderar comentarios directamente desde las entradas de tu blog de WordPress.

<sup>(Opcional)</sup> Recuerda también añadir a tus administradores en [Usuarios & Administrators](https://fastcomments.com/auth/my-account/users) y a los moderadores en [Moderadores de comentarios](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
para mejorar su experiencia y habilitar el seguimiento de estadísticas para moderadores.

El SSO se puede habilitar yendo al panel del plugin y haciendo clic en "Ajustes de SSO".

Primero tendrás que habilitar la función "Anyone can Register" de tu sitio.

Toda la información del usuario se transfiere de manera transparente y segura a FastComments cada vez que un usuario ve un hilo de comentarios mediante [HMAC](https://en.wikipedia.org/wiki/HMAC).

No es necesario ejecutar ninguna sincronización inicial o continua para copiar tus miembros a los servidores de FastComments. Esto se hace automáticamente cuando ven hilos de comentarios al abrir una entrada del blog.

## Nombres y avatares

El plugin actualizará automáticamente el nombre para mostrar del usuario y el avatar en todos sus comentarios dentro de FastComments cuando vean cualquier hilo de comentarios. Los avatares son compatibles mediante Gravatar o cualquier plugin de gestión de avatares dentro de WordPress, ya que el plugin llamará a `get_avatar_url`.
---