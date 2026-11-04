Si estás migrando tu sitio fuera de WordPress y quieres FastComments en el nuevo sitio, no necesitas el plugin de WordPress. Exporta tus comentarios desde
WordPress y luego sube el archivo en la [página de Importación](https://fastcomments.com/auth/my-account/manage-data/import) del panel de FastComments.

Soportamos dos formatos de exportación de WordPress.

### WordPress XML (Recomendado)

Este es el archivo del exportador integrado de WordPress, por lo que no se necesita ningún plugin adicional.

1. En tu administrador de WordPress, ve a `Tools -> Export`.
2. Selecciona `All content` y haz clic en `Download Export File`.
3. En la [página de Importación](https://fastcomments.com/auth/my-account/manage-data/import) de FastComments, selecciona `WordPress (.xml)` y sube el archivo.

Cada comentario está vinculado a la URL de la publicación en la que se dejó, la cual ya está incluida en el archivo.

La importación conserva el nombre del autor, el correo electrónico y el sitio web, la fecha, el contenido, los hilos de respuestas y si el comentario fue aprobado. Los avatares de los comentaristas se
traen desde Gravatar. Los votos no forman parte de este formato.

### WordPress CSV

Este es el archivo del [plugin WebToffee's WordPress Comments Import & Export](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Instala el plugin en tu administrador de WordPress y exporta tus comentarios como CSV.
2. Reemplaza cada valor de `comment_post_ID` con la URL de la publicación.
3. En la [página de Importación](https://fastcomments.com/auth/my-account/manage-data/import) de FastComments, selecciona `WordPress (.csv)` y sube el archivo.

Cada comentario está vinculado a la columna `comment_post_ID`. WordPress llena esta columna con el ID de la publicación, y tu nuevo sitio no tiene IDs de publicaciones de WordPress,
por lo que el paso 2 lo reemplaza con la URL.

La importación conserva el nombre del autor, el correo electrónico y el sitio web, la fecha, el contenido, los hilos de respuestas y si el comentario fue aprobado. Los avatares de los comentaristas se
traen desde Gravatar. También conserva la marca de spam de WordPress, y los likes y dislikes de wpDiscuz cuando el archivo los incluye.

### Asociar Comentarios a Tus Nuevas Páginas

Si tu nuevo sitio mantiene las mismas URLs que tu sitio de WordPress, los comentarios aparecerán en las páginas correspondientes sin configuración adicional.

Si el dominio cambia, ejecuta la [herramienta de Migración de Dominio](/guide-migrations.html#migrating-domains) después de la importación. Si las URLs de páginas individuales cambian, puedes
[migrar cada página](/guide-migrations.html#migrating-pages) desde su URL antigua a la nueva.

Para migraciones de páginas en masa, como eliminar el dominio del valor que pasas al campo [urlId](/guide-customizations-and-configuration.html#url-id)
del widget de comentarios, [abre un ticket de soporte](https://fastcomments.com/auth/my-account/help) y lo manejaremos por ti.

### Antes de Cambiar

Puedes ejecutar la importación tantas veces como quieras. Reimportar el mismo archivo [no crea duplicados](/guide-migrations.html#importing-data), por lo que puedes
importar una vez para probar el nuevo sitio, y luego importar de nuevo con tus últimos comentarios justo antes del cambio.

Para archivos de exportación mayores de 1GB, [contacta al soporte](https://fastcomments.com/auth/my-account/help).

Para agregar FastComments a tu nuevo sitio, consulta la [guía de Instalación](/guide-installation.html).