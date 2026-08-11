[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

El recuento de comentarios que se muestra en la parte superior del widget de comentarios se puede personalizar.

Esto puede reemplazarse con cualquier cadena, y el valor **[count]** será sustituido por el valor del recuento, localizado para el usuario.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Personalizando el texto del recuento de comentarios'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Campo de texto del recuento de comentarios en la página de personalización del widget, donde [count] se reemplaza con el total en vivo'; title='Personalizando el texto del recuento de comentarios' app-screenshot-end]