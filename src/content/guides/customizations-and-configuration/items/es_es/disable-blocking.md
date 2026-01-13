[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments permite a los usuarios bloquear a otros usuarios. Bloquear a un usuario hará que sus comentarios
se enmascaren, evitará las notificaciones entre los usuarios, y así sucesivamente.

Puede ser deseable desactivar esta funcionalidad. Se puede hacer de la siguiente manera:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Esto también se puede hacer sin código, lo que además habilita la validación adecuada del lado del servidor, a través de la interfaz de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---