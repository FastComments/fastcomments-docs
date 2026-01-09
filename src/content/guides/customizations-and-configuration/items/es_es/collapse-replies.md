[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Por defecto, las respuestas a los comentarios de nivel superior se muestran.

Esto se puede configurar para que el usuario tenga que hacer clic en "Mostrar respuestas" en los comentarios de nivel superior para ver las respuestas hijas.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Esta configuración no afectará el número de comentarios de nivel superior cargados inicialmente. Si tienes un comentario de nivel superior, y 29 hijos, con esta opción activada verás:

- Verás el comentario de nivel superior.
- Verás "Mostrar respuestas (29)" debajo de este comentario.

Si deseas mostrar todos los comentarios de nivel superior en combinación con esta opción, establece [la página inicial en -1](#starting-page).

---