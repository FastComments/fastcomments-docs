[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará una etiqueta "Comentario no verificado" para los comentarios que se hayan dejado para un usuario que tenga una sesión de navegador no verificada. Lea más sobre los comentarios no verificados [aquí](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Desactivar la etiqueta de comentario no verificado'; code-example-end]

Además, esta función se puede usar, sin escribir código, en la interfaz de personalización:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Página de personalización del widget con la casilla Desactivar la etiqueta de comentario no verificado marcada'; title='Desactivar la etiqueta de comentario no verificado' app-screenshot-end]