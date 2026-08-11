[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Para la autenticación, FastComments depende de que las cookies de terceros estén habilitadas en su navegador. Sin ellas, los usuarios siempre tendrán que
dejar su correo electrónico para comentar (a menos que el campo de entrada de correo esté oculto), y sus comentarios siempre se mostrarán como no verificados (por defecto).

Para evitar esto, puede habilitar la derivación de cookies de terceros. 

Cuando esta configuración está habilitada, se mostrará una pequeña ventana emergente que indica que el usuario está iniciando sesión. Esta ventana emergente
aparece siempre que el usuario interactúa con el widget de comentarios; por ejemplo, si deja un comentario.

Podemos hacer esto en código estableciendo la **enableThirdPartyCookieBypass** a true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Activar la omisión de cookies de terceros'; code-example-end]

También podemos configurar esto a través de la UI de Personalización del Widget, bajo `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Página de personalización del widget con la casilla de verificación de habilitar el popup de cookies de terceros marcada'; title='Activar la omisión de cookies de terceros' app-screenshot-end]