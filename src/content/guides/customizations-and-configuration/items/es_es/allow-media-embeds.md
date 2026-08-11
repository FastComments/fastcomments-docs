Por defecto FastComments no permite iframes en los comentarios. Cuando habilitas incrustaciones de medios, los comentaristas pueden pegar el código de incrustación (el fragmento `<iframe>`) de proveedores de confianza como YouTube, Vimeo, SoundCloud y Spotify, y se mostrará en línea dentro del comentario.

Por razones de seguridad, esto no es una bandera de configuración del widget del lado del cliente. Es una configuración del lado del servidor, validada cuando se guarda cada comentario, por lo que no se puede activar desde la página. Sólo se permiten iframes que apunten a una lista incorporada de proveedores de confianza. Cualquier otro iframe se elimina.

Esto se hace sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Configuración de incrustaciones de medios activada en la página de personalización del widget, permitiendo a los comentaristas pegar incrustaciones de iframe de confianza'; title='Permitir incrustaciones de medios' app-screenshot-end]

### Añadiendo tus propios proveedores

Si deseas permitir incrustaciones de un proveedor que no está en la lista de confianza incorporada, agrega su nombre de host en el campo "Dominios de incrustación adicionales" en la misma página. Estos nombres de host se permiten además de los proveedores incorporados. La coincidencia es exacta, así que incluye el nombre de host completo (por ejemplo, player.example.com). Cualquier cosa que no listes permanecerá bloqueada.

Tanto el cuadro de comentario simple como el editor WYSIWYG admiten pegar una incrustación. En el editor WYSIWYG la incrustación se inserta como un bloque removible.