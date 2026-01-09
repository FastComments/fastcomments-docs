[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Para la autenticación, FastComments depende de que las cookies de terceros estén habilitadas en su navegador. Sin ellas, los usuarios siempre tendrán que
dejar su email para comentar (a menos que el campo de entrada de email esté oculto), y sus comentarios siempre aparecerán como no verificados (por defecto).

Para solucionarlo, puede habilitar la omisión de cookies de terceros. 

Cuando esta configuración está habilitada, provocará una pequeña ventana emergente que muestra un mensaje indicando que el usuario está iniciando sesión. Esta ventana emergente
se muestra cada vez que el usuario interactúa con el widget de comentarios; por ejemplo, si dejan un comentario.

Podemos hacer esto en el código estableciendo la bandera **enableThirdPartyCookieBypass** a true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

También podemos configurar esto a través de la interfaz de personalización del widget, en `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---