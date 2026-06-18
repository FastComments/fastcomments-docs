Por defecto FastComments no permite iframes en los comentarios. Cuando habilitas los incrustados de medios, los comentaristas pueden pegar el código de incrustación (el `<iframe>` snippet) de proveedores de confianza como YouTube, Vimeo, SoundCloud y Spotify, y se renderizará en línea dentro del comentario.

Por seguridad, esto no es una bandera de configuración del widget del lado del cliente. Es una configuración del lado del servidor, validada cuando se guarda cada comentario, por lo que no puede activarse desde la página. Solo se permiten iframes que apunten a una lista integrada de proveedores de confianza. Cualquier otro iframe se elimina.

Esto se hace sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Añadir tus propios proveedores

Si quieres permitir incrustaciones de un proveedor que no está en la lista integrada de confianza, añade su nombre de host en el campo "Additional Embed Domains" en la misma página. Estos nombres de host se permiten además de los proveedores integrados. La coincidencia es exacta, así que incluye el nombre de host completo (por ejemplo, player.example.com). Cualquier cosa que no listes permanecerá bloqueada.

Tanto el cuadro de comentario sencillo como el editor WYSIWYG admiten pegar una incrustación. En el editor WYSIWYG la incrustación se inserta como un bloque que se puede eliminar.