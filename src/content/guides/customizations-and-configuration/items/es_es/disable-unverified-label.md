[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará una etiqueta "Comentario no verificado" para los comentarios que se hayan dejado a un usuario que
tiene una sesión de navegador no verificada. Más información sobre los comentarios no verificados [aquí](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Además, esta función se puede usar, sin escribir código, en la interfaz de personalización:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---