[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Al renderizar un hilo de comentarios, o al dejar un comentario, FastComments necesita saber a qué página, artículo o producto
pertenecen esos comentarios.

Para ello, usamos algo que llamamos el "URL ID". Es ya sea un identificador, como una cadena o un número, o una URL.

Por defecto, si no especificas el urlId, este será la URL de la página. Tomaremos la URL actual de la página y la limpiaremos para eliminar
cualquier parámetro común de marketing o identificadores de seguimiento.

En el caso de integraciones de terceros, como WordPress, nuestro plugin normalmente usará el identificador que representa la información actual que se está viendo como
el URL ID, por ejemplo el id del artículo/página.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Una cosa a la que nos referiremos a menudo en este documento es la <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Interfaz de personalización del widget</a>.

Esta interfaz puede usarse para realizar muchos cambios en el widget de comentarios sin utilizar código.

Al crear una regla de personalización, a menudo querremos que se aplique a todas las páginas de nuestro sitio. Sin embargo, en algunos casos queremos personalizar el widget de comentarios
en una página en particular, ya sea para aplicar estilos personalizados, o quizá para que los comentarios de esa página en particular sean anónimos. También podrías, por ejemplo, hacer que los comentarios en vivo
aparezcan de inmediato en algunas páginas, mientras que en otras se oculten bajo botones de notificación.

Todo esto es posible a través del campo de entrada URL ID en esta página, que se muestra de la siguiente manera:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

El valor en este campo debe coincidir con el parámetro *urlId* pasado al widget de comentarios. Si quieres que tu regla de personalización sea agnóstica al *urlId*, deja este campo vacío o introduce *.

A partir de 2023 el campo `URL ID` en la personalización del widget ahora también acepta patrones. Por ejemplo, puedes
tener `*/blog/*` para añadir estilos específicos a tu blog y `*/store/*` para tener estilos específicos en tu tienda,
todo mientras usas el mismo dominio.

### Advertencias

1. Si tu página tiene parámetros de fragmento (como example.com#page-1) - esto se convertirá en parte del URL ID, por defecto.
2. Durante migraciones, por ejemplo de WordPress a Gatsby, puede que tengas que migrar los valores de comentarios del URL ID después de la migración inicial. Para eso, contáctanos.

---