[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará una pestaña de "Mensajes Directos" en los perfiles de usuario, permitiendo a los visitantes enviar mensajes directos a un usuario.

Sin embargo, podemos desactivar esta pestaña:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, vea la sección "Disable Direct Messages".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='Página de personalización del widget con la casilla Desactivar mensajes directos marcada para ocultar la pestaña de mensajes del perfil'; title='Desactivar mensajes directos del perfil' app-screenshot-end]