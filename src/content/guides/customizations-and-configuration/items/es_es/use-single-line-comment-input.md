[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments permitirá al usuario ingresar un comentario con tantas líneas como desee, hasta el límite de caracteres predeterminado.

Sin embargo, puede ser deseable limitar al usuario a ingresar solo una línea de texto. Algunos casos de uso de ejemplo incluyen pujas en línea o chat en vivo, para los cuales se puede usar FastComments.

Activamos la bandera **useSingleLineCommentInput** de la siguiente manera:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Habilitar entrada de comentario de una sola línea'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, vea la sección "Habilitar entrada de comentario de una sola línea".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Casilla de verificación de entrada de comentario de una sola línea activada en la página de personalización del widget, limitando la entrada a una línea'; title='Habilitar entrada de comentario de una sola línea' app-screenshot-end]

Tenga en cuenta que los comentarios en cada página para cada dirección de ordenación se pre‑calculan, por lo que todas las direcciones de ordenación tienen el mismo rendimiento.