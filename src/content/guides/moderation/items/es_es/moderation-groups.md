Los moderadores pueden asignarse a grupos para moderar distintas páginas o categorías de contenido.

Cuando un moderador pertenece a uno o más grupos, solo verá los comentarios de esos grupos en la página Moderar comentarios.

Por ejemplo, supongamos que gestionamos un sitio que muestra vídeos por categoría. Podemos querer tener moderadores diferentes para vídeos de gatos, perros y loros, así que [añadamos esos grupos](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

Al añadir un moderador, ahora tenemos la opción de seleccionar uno o más grupos a los que pertenecerá el moderador:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

Finalmente, los comentarios deben vincularse a uno o más grupos para que los moderadores correctos los vean.

Esto se puede configurar [añadiendo algunos grupos](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) y luego especificando los correspondientes `Moderation Group` ids en el widget de comentarios,
[como se indica aquí](/guide-customizations-and-configuration.html#moderation-group-ids).

---