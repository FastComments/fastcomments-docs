[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Por defecto, las respuestas a los comentarios de nivel superior se muestran.

Esto se puede configurar para que el usuario tenga que hacer clic en "Mostrar respuestas" en los comentarios de nivel superior para ver los hijos.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Opción de colapsar respuestas en la interfaz de personalización del widget, ocultando los comentarios hijos detrás de un enlace Mostrar respuestas'; title='Colapsar respuestas' app-screenshot-end]

Esta configuración no afectará la cantidad de comentarios de nivel superior cargados inicialmente. Si tienes un comentario de nivel superior y 29 respuestas, con esta configuración activada, tú:

- Ver el comentario de nivel superior.
- Ver "Mostrar respuestas (29)" bajo este comentario.

Si deseas mostrar todos los comentarios de nivel superior en combinación con esta opción, establece [página inicial a -1](#starting-page).