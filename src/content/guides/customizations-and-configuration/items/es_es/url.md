[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Al enviar correos electrónicos de notificación, o al renderizar comentarios en interfaces de usuario como la página de moderación, es útil poder enlazar
desde el comentario a la página en la que se encuentra.

Si el ID de URL no es siempre un ID, entonces tenemos que almacenar la URL en otro lugar. Para eso sirve la propiedad "url", definida como sigue.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Un caso de uso común es vincular el hilo de comentarios a un identificador, como un artículo, y luego enlazar de vuelta a una página en particular, por ejemplo:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

La URL no se limpia de parámetros de marketing comunes. Por defecto, sea cual sea la URL de la página actual, esa es la URL que se guarda con el comentario.