---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Por defecto, el widget de comentarios de FastComments establecerá un `gif rating` de `pg`.

Las opciones disponibles son `g`, `pg`, `pg-13` y `r`.

Esto se puede configurar en el código o mediante la interfaz de usuario. En el código, podemos hacerlo de la siguiente manera:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

En la interfaz, encontrarás esto bajo `Gif Picker Rating` siempre que `Disable Image Uploads?` no esté marcado.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---