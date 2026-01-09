[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments permitirá a los usuarios ingresar un comentario con tantas líneas como deseen, hasta el límite de caracteres predeterminado.

Sin embargo, puede ser deseable limitar al usuario a ingresar solo una línea de texto. Algunos ejemplos de casos de uso incluyen subastas en línea, o chat en vivo, para los cuales FastComments
puede utilizarse.

Activamos la bandera **useSingleLineCommentInput** de la siguiente manera:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, consulte la sección "Habilitar entrada de comentario de una sola línea".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Tenga en cuenta que, los comentarios en cada página para cada dirección de ordenamiento se precomputan, por lo que todas las direcciones de ordenamiento tienen el mismo rendimiento.