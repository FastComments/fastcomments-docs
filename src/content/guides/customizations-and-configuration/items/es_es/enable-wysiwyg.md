[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Por defecto, las funcionalidades de formato en FastComments se realizan añadiendo etiquetas de anclaje visibles como `<b></b>` alrededor de su texto. Hacer clic en la barra de herramientas
or using shortcuts does this for you. Sin embargo, algunas comunidades pueden querer optar por usar formato sin etiquetas de anclaje. Esto se llama habilitar el
editor WYSIWYG (what you see is what you get). Este editor se ve exactamente igual que el predeterminado, excepto que carga código adicional que permite a los usuarios poner en negrita, subrayar, etc. su texto sin etiquetas de anclaje visibles.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Habilitando la edición WYSIWYG'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, vea la opción "Habilitar formato avanzado".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Página de personalización del widget con la casilla Habilitar formato avanzado marcada para activar el editor WYSIWYG'; title='Habilitar WYSIWYG' app-screenshot-end]