[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Por defecto, el widget de comentarios de FastComments establecerá una `gif rating` de `pg`.

Las opciones disponibles son `g`, `pg`, `pg-13` y `r`.

Esto se puede establecer en el código o a través de la UI. En el código podemos hacerlo de la siguiente manera:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Establecer clasificación de Gif'; code-example-end]

En la UI, encontrarás esto bajo `Gif Picker Rating` siempre que `Disable Image Uploads?` no esté marcado.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Desplegable de Gif Picker Rating en la página de personalización del widget que ofrece g, pg, pg-13 y r'; title='Configurando la clasificación de Gif' app-screenshot-end]