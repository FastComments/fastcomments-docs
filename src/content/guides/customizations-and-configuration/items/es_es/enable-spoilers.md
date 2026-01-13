[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Podemos habilitar el soporte de spoilers estableciendo la bandera **enableSpoilers** en true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, vea la opción "Activar spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Cuando se resalta texto, y se hace clic en el botón `SPOILER` que ahora es visible, el texto quedará enmascarado hasta que el usuario sitúe el cursor sobre él. Para el modo oscuro hacemos lo mismo, con diferentes
colores que se ajustan mejor al modo oscuro.

Esto también es compatible con el editor WYSIWYG.

---