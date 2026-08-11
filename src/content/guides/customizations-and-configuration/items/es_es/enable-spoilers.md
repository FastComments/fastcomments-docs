[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Podemos habilitar el soporte de spoilers estableciendo la bandera **enableSpoilers** a true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Habilitando Spoilers'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, vea la opción "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Página de personalización del widget con la casilla Enable Spoilers marcada para añadir el botón SPOILER al editor'; title='Habilitar Spoilers' app-screenshot-end]

Cuando el texto está resaltado y se hace clic en el botón `SPOILER` ahora visible, el texto se enmascarará hasta que el usuario pase el cursor sobre él. Para el modo oscuro hacemos lo mismo, con colores diferentes que se adaptan mejor al modo oscuro.

Esto también es compatible con el editor WYSIWYG.