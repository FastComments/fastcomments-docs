[related-parameter-start name = 'headerHTML'; type = 'string'; related-parameter-end]

Algún texto, como un encabezado o mensaje, puede mostrarse debajo del recuento de comentarios pero encima del texto del estado de inicio de sesión.

Llamamos a esto el encabezado, y por defecto está oculto.

[code-example-start config = {headerHTML: "<h1>Leave a Comment!</h1>"}; linesToHighlight = [6]; title = 'Specifying Header HTML'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, bajo Opciones Avanzadas:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.absolute-dates'; alt='Área de Opciones Avanzadas de la página de personalización del widget donde se ingresa HTML de encabezado personalizado'; title='Especificar HTML de encabezado' app-screenshot-end]