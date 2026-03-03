[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments mostrará una pestaña "Mensajes Directos" en los perfiles de usuario, lo que permite a los visitantes enviar mensajes directos a un usuario.

Sin embargo, podemos desactivar esta pestaña:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, consulta la sección "Desactivar Mensajes Directos".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; title='Disable Profile Direct Messages' app-screenshot-end]