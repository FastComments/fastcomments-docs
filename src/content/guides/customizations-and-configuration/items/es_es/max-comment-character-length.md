[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

El número máximo de caracteres permitidos para ingresar en el campo de entrada de comentarios puede ser limitado por el parámetro **maxCommentCharacterLength**.

El valor predeterminado es 2000.

Elementos como URLs de imágenes no se incluyen en la determinación de la longitud.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Campo de tamaño máximo de comentario en la página de personalización del widget, usado para limitar cuántos caracteres puede contener un comentario'; title='Limitar longitud del comentario' app-screenshot-end]