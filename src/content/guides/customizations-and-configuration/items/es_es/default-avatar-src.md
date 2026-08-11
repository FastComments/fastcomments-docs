[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Cuando un usuario comenta con FastComments por primera vez, intentaremos obtener su avatar de <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Sin embargo, si no encontramos un avatar, o el usuario nunca establece uno en su cuenta, mostramos una imagen de avatar predeterminada estática.

Para especificar tu propia imagen de avatar estática, puedes usar la configuración *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, consulta la sección "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Sección Avatar predeterminado de la página de personalización del widget, donde estableces la URL de la imagen de avatar de reserva'; title='Personalizando el Avatar predeterminado' app-screenshot-end]

Ten en cuenta que la definición del avatar para un usuario en particular, como con SSO, se cubre en su propia sección.