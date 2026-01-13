[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Por defecto, las funcionalidades de formato en FastComments se realizan añadiendo etiquetas ancla visibles como `<b></b>` alrededor de tu texto. Al hacer clic en la barra de herramientas
o usar atajos, esto se hace por ti. Sin embargo, algunas comunidades pueden querer optar por usar formato sin etiquetas ancla visibles. Esto se llama habilitar el
WYSIWYG (lo que ves es lo que obtienes) editor. Este editor se ve exactamente igual que el predeterminado, excepto que carga algo de
código adicional que permite a los usuarios poner texto en negrita, subrayar, etc., sin etiquetas ancla visibles.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Esto también se puede hacer sin código. En la página de personalización del widget, consulte la opción "Habilitar formato avanzado".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]