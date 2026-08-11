[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments permite a los usuarios bloquear a otros usuarios. Bloquear a un usuario hará que sus comentarios se oculten, evita notificaciones entre los usuarios, y así sucesivamente.

Puede ser deseable desactivar esta funcionalidad. Se puede hacer de la siguiente manera:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Esto también se puede hacer sin código, lo que también permite una validación adecuada del lado del servidor, a través de la interfaz de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Opción de desactivar el bloqueo en la interfaz de personalización del widget, que impide que los usuarios se bloqueen entre sí'; title='Desactivar bloqueo' app-screenshot-end]