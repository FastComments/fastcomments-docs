[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Al renderizar un hilo de comentarios, o al dejar un comentario, FastComments necesita saber a qué página, artículo o producto pertenecen esos comentarios.

Para ello, utilizamos algo que llamamos "URL ID". Es un identificador, ya sea una cadena o un número, o una URL.

Por defecto, si no especificas el urlId, se convertirá en la URL de la página. Tomaremos la URL actual de la página y la limpiaremos para eliminar cualquier parámetro de marketing común o identificadores de seguimiento.

En el caso de integraciones de terceros, como WordPress, nuestro plugin suele usar el identificador que representa la información actual que se está viendo como el URL ID, por ejemplo el id del artículo/página.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definir un ID de URL personalizado'; code-example-end]

Una cosa a la que a menudo haremos referencia en este documento es la <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Interfaz de Personalización del Widget</a>.

Esta interfaz se puede usar para realizar muchos cambios en el widget de comentarios sin usar código.

Al crear una regla de personalización, a menudo querremos que se aplique a todas las páginas de nuestro sitio. Sin embargo, en algunos casos queremos personalizar el widget de comentarios en una página concreta, ya sea para aplicar estilos personalizados o quizá hacer que los comentarios de esa página sean anónimos. También podrías, por ejemplo, hacer que los comentarios en vivo aparezcan de inmediato en algunas páginas, mientras que en otras se oculten bajo botones de notificación.

Todo esto es posible mediante el campo de entrada URL ID en esta página, que se muestra de la siguiente manera:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Campo ID de URL usado para aplicar una regla de personalización a una página, o a un patrón como */blog/*'; title='Entrada de ID de URL en la página de personalización del widget' app-screenshot-end]

El valor en este campo debe coincidir con el parámetro *urlId* que se pasa al widget de comentarios. Si deseas que tu regla de personalización sea independiente del *urlId*, deja este campo vacío o ingresa *.

A partir de 2023, el campo `URL ID` en la personalización del widget también acepta patrones. Por ejemplo, puedes usar `*/blog/*` para añadir estilos específicos a tu blog y `*/store/*` para estilos específicos a tu tienda, todo mientras utilizas el mismo dominio.

### Cosas a tener en cuenta

1. Si tu página tiene parámetros de hash (como example.com#page-1), esto se convertirá en parte del URL ID, por defecto.
2. Durante migraciones, por ejemplo de WordPress a Gatsby, puede que necesites migrar los valores de comentarios del URL ID después de la migración inicial. Para ello, contáctanos.