[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Para proyectos de personalización de estilos más grandes, puede ser deseable empezar de cero y no usar el estilo predeterminado en absoluto.

Todo el estilo predeterminado puede eliminarse estableciendo el parámetro **noStyles** en true, como sigue:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Esto se puede personalizar sin código, en la página de personalización del widget, en Opciones avanzadas:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]