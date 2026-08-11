[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Por defecto, FastComments solo solicitará al usuario su comentario, su nombre de usuario y su correo electrónico.

Sin embargo, en algunas situaciones puede que desees que el usuario deje un enlace a su propio blog o sitio web.

Podemos habilitar la visualización de un campo de entrada adicional para que el usuario ingrese la URL de su sitio web estableciendo la bandera **enableCommenterLinks** a true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Cuando se proporciona dicha URL, la cuenta del usuario se actualizará y todos sus nombres de usuario en comentarios pasados y futuros enlazarán a esa URL.

Esto se puede personalizar sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Página de personalización del widget con la casilla de verificación de enlaces de comentarista marcada para agregar un campo de URL del sitio web al formulario de comentario'; title='Habilitar enlaces de comentarista' app-screenshot-end]