## Acciones y Búsquedas

Las acciones crean datos en FastComments; las búsquedas buscan datos para que un paso posterior pueda usarlos. Cada acción llama a la API REST de FastComments y consume los mismos créditos de API que costaría la llamada desde su propio código: un crédito por llamada a menos que se indique lo contrario.

## Crear Comentario

Publica un comentario en una página.

| Campo | Obligatorio | Notas |
|-------|--------------|-------|
| Page URL ID | Sí | El ID de URL que el widget de comentarios usa en la página. Los comentarios se agrupan por él. |
| Page URL | Sí | La URL completa de la página, usada en los correos de notificación. |
| Comment | Sí | El cuerpo del comentario en markdown de FastComments. |
| Commenter Name | Sí | Los nombres son únicos por correo electrónico, por lo que reutilizar un nombre con un correo diferente falla. |
| Commenter Email | No | Se crea un usuario para el correo cuando aún no existe. |
| User ID | No | Un ID de usuario SSO existente. Tiene precedencia sobre el nombre y el correo. |
| Parent Comment ID | No | Establecer para publicar una respuesta. |
| Approved, Verified | No | Ambos predeterminados a verdadero. Los comentarios no aprobados permanecen ocultos hasta ser moderados. |
| Posted At | No | Predeterminado a ahora. |
| Avatar URL, Page Title, Locale | No | Locale predeterminado a `en_us`. |
| Show Live In Widget | No | Envía el comentario a los espectadores en tiempo real. Cuesta 2 créditos en lugar de 1. |
| Run Spam Check, Send Emails | No | Desactivado por defecto. |

## Crear Página

Crea un registro de página antes de que exista cualquier comentario en ella, para que pueda ser listada y restringida. Recibe el ID de URL, el título, la URL y, opcionalmente, los IDs de grupos SSO que pueden verla.

## Crear Usuario SSO

Crea un usuario de inicio de sesión único. Recibe su propio ID de usuario, nombre de usuario y correo electrónico, además de nombre para mostrar opcional, etiqueta de visualización, avatar, sitio web, IDs de grupos y banderas de notificación y privacidad. Los roles administrativos no pueden concederse desde Zapier.

## Crear Publicación en Feed

Crea una publicación en un feed de FastComments a partir de contenido HTML. Se requiere el ID de usuario del autor (un ID de usuario FastComments o SSO); el título, las etiquetas y una vista previa de enlace son opcionales.

## Crear Etiqueta Hash

Crea una etiqueta hash que los comentaristas pueden usar, con una URL opcional a la que enlaza. Las etiquetas son únicas por cuenta, por lo que un Zap que crea una en cada ejecución necesita algo único en la etiqueta.

## Señalar Comentario

Marca un comentario para revisión del moderador. Se requiere el ID del usuario que realiza la señalación; el ID del autor devuelto por Crear Comentario funciona.

## Búsquedas

| Búsqueda | Entrada | Devuelve |
|----------|---------|----------|
| Buscar Comentario | Comment ID | El comentario, o nada. |
| Buscar Usuario SSO | Email | El usuario SSO, o nada. |
| Buscar Página | URL ID | La página, o nada. |

Una búsqueda que no encuentra nada no falla el Zap. Combine una búsqueda con una creación en el modo "buscar o crear" de Zapier para crear la página o el usuario cuando falta.