[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Cuando un usuario comenta con FastComments por primera vez intentaremos obtener su avatar de <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Sin embargo, si no encontramos un avatar, o el usuario nunca configura uno en su cuenta, mostramos una imagen de avatar predeterminada estática.

Para especificar tu propia imagen de avatar estática, podemos usar la configuración *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, consulta la sección "Avatar predeterminado".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Ten en cuenta que definir el avatar para un usuario en particular, como con SSO, se trata en su propia sección.