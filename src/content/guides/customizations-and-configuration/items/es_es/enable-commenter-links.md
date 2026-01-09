[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments solo pedirá al usuario su comentario, su nombre de usuario y su correo electrónico.

Sin embargo, en algunas situaciones es posible que desee que el usuario deje un enlace a su propio blog o sitio web.

Podemos habilitar la visualización de un campo de entrada adicional para dejar la URL del sitio web del usuario estableciendo la bandera **enableCommenterLinks** en true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Cuando se proporciona dicha URL, la cuenta del usuario se actualizará y el nombre de usuario en todos sus comentarios, tanto pasados como futuros, enlazará a esta URL.

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---