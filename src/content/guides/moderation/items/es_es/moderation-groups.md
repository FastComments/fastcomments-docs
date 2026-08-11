---
Los moderadores pueden ser asignados a grupos para moderar diferentes páginas o categorías de contenido.

Cuando un moderador pertenece a uno o más grupos, solo verá los comentarios de esos grupos en la página Moderar Comentarios.

Por ejemplo, supongamos que administramos un sitio que muestra videos por categoría. Podríamos querer tener diferentes moderadores para videos de Gato, Perro y Loro, así que [agreguemos esos grupos](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Lista de grupos de moderación con los grupos Gato, Perro y Loro creados para cada categoría de video'; title='The Moderation Groups Page' app-screenshot-end]

Cuando agregamos un moderador, ahora tenemos la opción de seleccionar uno o más grupos a los que pertenecerá el moderador:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Formulario para agregar un moderador con el selector de grupo usado para asignar el moderador a uno o más grupos'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

Finalmente, los comentarios deben estar vinculados a uno o más grupos para que los moderadores correctos los vean.

Esto se puede configurar [agregando algunos grupos](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) y luego especificando los IDs de `Moderation Group` correspondientes en el widget de comentarios,
[como se indica aquí](/guide-customizations-and-configuration.html#moderation-group-ids).
---