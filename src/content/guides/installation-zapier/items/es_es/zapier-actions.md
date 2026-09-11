## Acciones y Búsquedas

Las acciones crean datos en FastComments; las búsquedas buscan datos para que un paso posterior pueda utilizarlos. Cada acción llama a la API REST de FastComments y consume los mismos créditos de API que costaría la llamada desde tu propio código: un crédito por llamada a menos que se indique lo contrario.

## Crear Comentario

Publica un comentario en una página.

| Campo | Requerido | Notas |
|-------|----------|-------|
| ID de URL de la página | Sí | El ID de URL que el widget de comentarios usa en la página. Los comentarios se agrupan por él. |
| URL de la página | Sí | La URL completa de la página, usada en los correos electrónicos de notificación. |
| Comentario | Sí | El cuerpo del comentario en markdown de FastComments. |
| Nombre del comentarista | Sí | Los nombres son únicos por correo electrónico, por lo que reutilizar un nombre con un correo diferente falla. |
| Correo del comentarista | No | Se crea un usuario para el correo cuando aún no existe. |
| ID de usuario | No | Un ID de usuario SSO existente. Tiene prioridad sobre el nombre y el correo. |
| ID del comentario padre | No | Establecer para publicar una respuesta. |
| Aprobado, Verificado | No | Ambos predeterminados a true. Los comentarios no aprobados permanecen ocultos hasta ser moderados. |
| Publicado en | No | Predeterminado a ahora. |
| URL del avatar, Título de la página, Locale | No | Locale predeterminado a `en_us`. |
| Mostrar en vivo en el widget | No | Envía el comentario a los espectadores en tiempo real. Cuesta 2 créditos en lugar de 1. |
| Ejecutar verificación de spam, Enviar correos | No | Desactivado por defecto. |

## Crear o Actualizar Página

Crea un registro de página antes de que exista cualquier comentario en ella, de modo que pueda listarse y restringirse. Toma el ID de URL, el título, la URL y, opcionalmente, los IDs de grupo SSO permitidos para verla. Si ya existe una página con ese ID de URL, se actualiza con los campos proporcionados, de modo que un Zap pueda ejecutarse repetidamente para la misma página.

## Crear o Actualizar Usuario SSO

Crea un usuario de inicio de sesión único. Toma tu propio ID de usuario, nombre de usuario y correo electrónico, más opcionalmente nombre para mostrar, etiqueta para mostrar, avatar, sitio web, IDs de grupo y banderas de notificación y privacidad. Si ya existe un usuario con ese ID, se actualiza en su lugar. No se pueden otorgar roles administrativos desde Zapier.

## Crear Publicación en Feed

Crea una publicación en un feed de FastComments a partir de contenido HTML. El ID de usuario autor es obligatorio (un ID de FastComments o SSO); el título, las etiquetas y una vista previa de enlace son opcionales.

## Crear o Actualizar Etiqueta Hash

Crea una etiqueta hash que los comentaristas pueden usar, con una URL opcional a la que enlaza. Si la etiqueta ya existe, se actualiza en su lugar.

## Señalar Comentario

Señala un comentario para revisión del moderador. Se requiere el ID del usuario que realiza la señalización; el ID del autor devuelto por Crear Comentario funciona.

## Búsquedas

| Búsqueda | Entrada | Devuelve |
|----------|---------|----------|
| Encontrar Comentario | ID del Comentario | El comentario, o nada. |
| Encontrar Usuario SSO | Correo electrónico | El usuario SSO, o nada. |
| Encontrar Página | ID de URL | La página, o nada. |

Una búsqueda que no encuentra nada no falla el Zap. Encontrar Usuario SSO y Encontrar Página ofrecen la opción de Zapier “crear si no existe”, que ejecuta la creación correspondiente cuando no se encuentra nada.