[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

El número máximo de caracteres permitidos en el campo de entrada de comentarios se puede limitar mediante el parámetro **maxCommentCharacterLength**.

El valor predeterminado es 2000.

Elementos como las URLs de imágenes no se incluyen en la determinación de la longitud.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]